// src/simulation.rs

use bevy::{
    prelude::*, 
    sprite::SpriteSheetBundle,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ui::node_bundles::AtlasImageBundle,
};
use crate::{
    AppState, GameAssets, level, 
    game::{Path, GameTile, TileType, get_tile_type, get_atlas_index, spawn_composite_tile},
    tower::{Tower, TowerType},
    enemy::{Enemy, Health}, // On n'utilise plus PathFollower du jeu normal
    projectile::Projectile,
};

// --- Composants & Ressources ---

#[derive(Component)]
struct SimComponent;

#[derive(Component)]
struct SimStatsText;

// Composant spécifique pour le mouvement en simulation (permet la boucle infinie)
#[derive(Component)]
struct SimPathFollower {
    path_index: usize,
}

#[derive(Resource)]
struct SimState {
    start_time: f64,
    total_spawned: u32,
    last_log_time: f64,
    spawn_rate: u32, // Nombre d'ennemis par frame
}

// --- Plugin ---

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Simulation), setup_simulation)
            .add_systems(Update, (
                spawn_massive_enemies, 
                move_sim_enemies_loop, // Mouvement avec boucle infinie
                update_performance_ui,
                log_performance_console
            ).run_if(in_state(AppState::Simulation)))
            .add_systems(OnExit(AppState::Simulation), cleanup_simulation);
    }
}

// --- Setup ---

fn setup_simulation(mut commands: Commands, assets: Res<GameAssets>) {
    println!("=== DÉMARRAGE BENCHMARK (Mode Stress Test Infini) ===");

    let level_data = level::get_level_data();
    const TILE_SIZE: f32 = 32.0;
    const MAP_WIDTH: f32 = 20.0 * TILE_SIZE;
    const MAP_HEIGHT: f32 = 20.0 * TILE_SIZE;
    let vertical_shift = 50.0;
    let x_offset = -MAP_WIDTH / 2.0 + TILE_SIZE / 2.0;
    let y_offset = (MAP_HEIGHT / 2.0 - TILE_SIZE / 2.0) + vertical_shift;

    let mut start_pos = Vec2::ZERO;

    // 1. Génération Carte + Tours
    for (y, row) in level_data.iter().enumerate() {
        for (x, &tile_id) in row.iter().enumerate() {
            let pos = Vec2::new(
                x_offset + x as f32 * TILE_SIZE,
                y_offset - y as f32 * TILE_SIZE,
            );

            let tile_type = get_tile_type(tile_id);
            
            // Helper spawn visuel
            let mut spawn_tile = |index: usize, rot: Quat, z: f32| {
                commands.spawn((
                    SpriteSheetBundle {
                        texture: assets.sprite_atlas.clone(),
                        atlas: TextureAtlas { layout: assets.sprite_atlas_layout.clone(), index },
                        transform: Transform { translation: pos.extend(z), rotation: rot, ..default() },
                        ..default()
                    },
                    SimComponent,
                ));
            };

            // Affichage Terrain
            match tile_id {
                8..=19 => {
                    spawn_tile(get_atlas_index(0, 0), Quat::IDENTITY, 0.0);
                    let (over_idx, rot) = get_composite_info(tile_id);
                    spawn_tile(over_idx, rot, 0.1);
                },
                20 | 21 => {
                    spawn_tile(get_atlas_index(8, 0), Quat::IDENTITY, 0.0);
                    let over_idx = if tile_id == 20 { get_atlas_index(7, 2) } else { get_atlas_index(8, 2) };
                    spawn_tile(over_idx, Quat::IDENTITY, 0.1);
                },
                _ => {
                    let (idx, rot) = get_simple_tile_info(tile_id);
                    spawn_tile(idx, rot, 0.0);
                }
            }

            if tile_id == 20 { start_pos = pos; }

            // Placement Tours (Toutes Tier 3)
            if tile_type == TileType::Grass {
                let tower_type = determine_sim_tower_type(x, y, level_data);
                let (range, damage, cooldown) = tower_type.get_sim_stats();

                commands.spawn((
                    SpriteSheetBundle {
                        texture: assets.sprite_atlas.clone(),
                        atlas: TextureAtlas { 
                            layout: assets.sprite_atlas_layout.clone(), 
                            index: tower_type.get_sprite_index() 
                        },
                        transform: Transform::from_xyz(pos.x, pos.y, 2.0),
                        ..default()
                    },
                    Tower {
                        range,
                        damage,
                        cooldown: Timer::from_seconds(cooldown, TimerMode::Repeating),
                    },
                    SimComponent,
                ));
            }
        }
    }

    // 2. Pathfinding
    let mut path_points = Vec::new();
    path_points.push(start_pos);
    let mut grid_x = ((start_pos.x - x_offset) / TILE_SIZE).round() as i32;
    let mut grid_y = ((y_offset - start_pos.y) / TILE_SIZE).round() as i32;
    let mut last_grid_pos = (grid_x, grid_y);

    for _ in 0..100 {
        let mut found_next = false;
        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in neighbors {
            let nx = grid_x + dx;
            let ny = grid_y + dy;
            if nx < 0 || ny < 0 || nx >= 20 || ny >= 20 { continue; }
            if (nx, ny) == last_grid_pos { continue; }

            let tid = level_data[ny as usize][nx as usize];
            let ttype = get_tile_type(tid);
            if ttype == TileType::Road || ttype == TileType::End {
                last_grid_pos = (grid_x, grid_y);
                grid_x = nx;
                grid_y = ny;
                path_points.push(Vec2::new(x_offset + nx as f32 * TILE_SIZE, y_offset - ny as f32 * TILE_SIZE));
                found_next = true;
                if ttype == TileType::End { break; }
                break;
            }
        }
        if !found_next { break; }
        if get_tile_type(level_data[grid_y as usize][grid_x as usize]) == TileType::End { break; }
    }
    commands.insert_resource(Path { points: path_points });

    // 3. UI
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        },
        SimComponent,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section("Init...", TextStyle { font_size: 20.0, color: Color::GREEN, ..default() }),
            SimStatsText,
        ));
    });

    // 4. État Initial
    commands.insert_resource(SimState {
        start_time: 0.0,
        total_spawned: 0,
        last_log_time: 0.0,
        spawn_rate: 10, // Commence à 10 par frame (600/sec)
    });
}

// --- Spawn Massif & Progressif ---

fn spawn_massive_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut sim_state: ResMut<SimState>,
    path: Res<Path>, // On a besoin du chemin pour la position de départ
    time: Res<Time>,
) {
    if sim_state.start_time == 0.0 { sim_state.start_time = time.elapsed_seconds_f64(); }
    if path.points.is_empty() { return; }

    // Augmentation progressive du taux de spawn (Optionnel : Stress Test)
    // Toutes les 10 secondes, on ajoute +1 ennemi par frame (+60/sec)
    let elapsed = time.elapsed_seconds_f64() - sim_state.start_time;
    let ramp_up = (elapsed / 10.0) as u32; 
    let current_spawn_rate = 10 + ramp_up; 
    sim_state.spawn_rate = current_spawn_rate;

    let start_pos = path.points[0];

    for _ in 0..current_spawn_rate {
        sim_state.total_spawned += 1;
        let hp = 85; // 85000 pour test extrême si besoin
        let speed = 50.0; // Rapide

        commands.spawn((
            SpriteSheetBundle {
                texture: assets.sprite_atlas.clone(),
                atlas: TextureAtlas { layout: assets.sprite_atlas_layout.clone(), index: 10 }, // Orc
                // CORRECTION 1 : Spawn direct au départ
                transform: Transform::from_xyz(start_pos.x, start_pos.y, 1.0), 
                ..default()
            },
            Enemy { speed },
            Health { current: hp, max: hp },
            // CORRECTION 2 : Utilise SimPathFollower pour le mouvement infini
            SimPathFollower { path_index: 1 }, 
            SimComponent,
        ));
    }
}

// --- Mouvement Infini (Boucle) ---

fn move_sim_enemies_loop(
    mut query: Query<(&mut Transform, &Enemy, &mut SimPathFollower)>,
    path: Res<Path>,
    time: Res<Time>,
) {
    if path.points.is_empty() { return; }

    for (mut transform, enemy, mut follower) in query.iter_mut() {
        // Si on dépasse la fin du chemin -> Retour case départ (Boucle infinie)
        if follower.path_index >= path.points.len() {
            follower.path_index = 1; // On vise le point 1
            let start = path.points[0];
            transform.translation.x = start.x;
            transform.translation.y = start.y;
            continue;
        }

        let target = path.points[follower.path_index];
        let direction = target - transform.translation.truncate();
        let distance = direction.length();
        let step = enemy.speed * time.delta_seconds();

        if distance <= step {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            follower.path_index += 1;
        } else {
            let movement = direction.normalize() * step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
        
        // Rotation simplifiée
        if direction.x.abs() > direction.y.abs() {
             if direction.x > 0.0 { transform.rotation = Quat::IDENTITY; } 
             else { transform.rotation = Quat::from_rotation_y(std::f32::consts::PI); }
        }
    }
}

// --- UI et Logging ---

fn update_performance_ui(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    sim_state: Res<SimState>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    mut text_query: Query<&mut Text, With<SimStatsText>>,
) {
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|f| f.smoothed()).unwrap_or(0.0);
    let active_enemies = enemies.iter().count();
    let active_projectiles = projectiles.iter().count();
    let elapsed = time.elapsed_seconds_f64() - sim_state.start_time;

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Temps: {:.1}s\nFPS: {:.1}\nEnnemis Actifs: {}\nTotal Spawnés: {}\nSpawn Rate: {}/frame\nProjectiles: {}",
            elapsed, fps, active_enemies, sim_state.total_spawned, sim_state.spawn_rate, active_projectiles
        );
        
        // Change la couleur si les FPS chutent
        if fps < 30.0 { text.sections[0].style.color = Color::RED; }
        else if fps < 55.0 { text.sections[0].style.color = Color::YELLOW; }
        else { text.sections[0].style.color = Color::GREEN; }
    }
}

fn log_performance_console(
    time: Res<Time>,
    mut sim_state: ResMut<SimState>,
    diagnostics: Res<DiagnosticsStore>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let now = time.elapsed_seconds_f64();
    if now - sim_state.last_log_time >= 1.0 {
        let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|f| f.smoothed()).unwrap_or(0.0);
        let count = enemies.iter().count();
        println!("PERF: {:.1}s, Total: {}, FPS: {:.1}, Actifs: {}", 
            now - sim_state.start_time, sim_state.total_spawned, fps, count);
        sim_state.last_log_time = now;
    }
}

fn cleanup_simulation(mut commands: Commands, query: Query<Entity, With<SimComponent>>) {
    for entity in query.iter() { commands.entity(entity).despawn_recursive(); }
    commands.remove_resource::<Path>();
    commands.remove_resource::<SimState>();
}

// --- Helpers Visuels (Identiques) ---
fn determine_sim_tower_type(x: usize, y: usize, level: &[[u32; 20]; 20]) -> TowerType {
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut next_to_road = false;
    for (dx, dy) in neighbors {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < 20 && ny < 20 {
            let tid = level[ny as usize][nx as usize];
            let ttype = get_tile_type(tid);
            if ttype == TileType::Road || ttype == TileType::Start || ttype == TileType::End {
                next_to_road = true;
                break;
            }
        }
    }
    if next_to_road { return TowerType::Canon; }
    TowerType::Archer
}

fn get_composite_info(id: u32) -> (usize, Quat) {
    let r90 = -90.0f32.to_radians(); let r180 = 180.0f32.to_radians(); let r270 = 90.0f32.to_radians();
    match id {
        8 => (get_atlas_index(5, 0), Quat::IDENTITY), 9 => (get_atlas_index(5, 0), Quat::from_rotation_z(r90)),
        10 => (get_atlas_index(5, 0), Quat::from_rotation_z(r180)), 11 => (get_atlas_index(5, 0), Quat::from_rotation_z(r270)),
        12 => (get_atlas_index(6, 0), Quat::IDENTITY), 13 => (get_atlas_index(6, 0), Quat::from_rotation_z(r90)),
        14 => (get_atlas_index(6, 0), Quat::from_rotation_z(r180)), 15 => (get_atlas_index(6, 0), Quat::from_rotation_z(r270)),
        16 => (get_atlas_index(4, 0), Quat::IDENTITY), 17 => (get_atlas_index(4, 0), Quat::from_rotation_z(r90)),
        18 => (get_atlas_index(4, 0), Quat::from_rotation_z(r180)), 19 => (get_atlas_index(4, 0), Quat::from_rotation_z(r270)),
        _ => (0, Quat::IDENTITY),
    }
}

fn get_simple_tile_info(id: u32) -> (usize, Quat) {
    let r90 = -90.0f32.to_radians(); let r180 = 180.0f32.to_radians(); let r270 = 90.0f32.to_radians();
    match id {
        0 => (get_atlas_index(9, 0), Quat::IDENTITY), 1 => (get_atlas_index(0, 0), Quat::IDENTITY),
        2 => (get_atlas_index(8, 0), Quat::IDENTITY), 3 => (get_atlas_index(8, 0), Quat::from_rotation_z(r90)),
        4 => (get_atlas_index(7, 0), Quat::IDENTITY), 5 => (get_atlas_index(7, 0), Quat::from_rotation_z(r90)),
        6 => (get_atlas_index(7, 0), Quat::from_rotation_z(r180)), 7 => (get_atlas_index(7, 0), Quat::from_rotation_z(r270)),
        _ => (get_atlas_index(0, 0), Quat::IDENTITY),
    }
}