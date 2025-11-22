use bevy::{prelude::*, sprite::SpriteSheetBundle, window::PrimaryWindow, ui::node_bundles::AtlasImageBundle,};
use crate::{
    AppState, 
    GameAssets,
    level,
    constants::tiles as TileTypes,
    tower::{Tower, TowerType},
    enemy::Enemy,
    projectile::Projectile,
};


// Composant pour tout ce qui est DANS le jeu
#[derive(Component)]
struct GameComponent;

// Équivalent de TileType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Water,
    Grass,
    Road,
    Start,
    End,
}

// Composant qui sera ajouté à chaque entité "Tuile"
#[derive(Component)]
pub struct GameTile {
    pub tile_type: TileType,
}

// Ressource pour stocker les points de départ et de fin
#[derive(Resource)]
pub struct Path {
    pub points: Vec<Vec2>, // Liste des points du chemin
}

// Ressource pour la tour sélectionnée dans le menu
#[derive(Resource, Default)]
struct SelectedTower {
    tower_type: Option<TowerType>,
}

// Pour les boutons de sélection de tours
#[derive(Component)]
struct TowerButton {
    tower_type: TowerType,
}

// Équivalent de "Playing.java"
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedTower>() // Initialise à None
            .add_systems(OnEnter(AppState::Playing), (setup_game, setup_game_ui)) // Ajout de l'UI
            .add_systems(Update, (tower_button_interaction, grid_click_interaction, tower_shooting).run_if(in_state(AppState::Playing)))
            .add_systems(OnExit(AppState::Playing), cleanup_game);
            
            
    }
}

fn get_atlas_index(x: usize, y: usize) -> usize {
    y * 10 + x
}

// équivalent constructeur Playing()
fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    println!("Lancement du jeu (Playing) !");

    // --- 1. Initialisation et Constantes ---
    let level_data = level::get_level_data();
    
    // Positions temporaires pour le pathfinding
    let mut start_pos = Vec2::ZERO;
    
    // Configuration de la grille
    const TILE_SIZE: f32 = 32.0;
    const MAP_WIDTH: f32 = 20.0 * TILE_SIZE;
    const MAP_HEIGHT: f32 = 20.0 * TILE_SIZE;
    
    // --- CORRECTION 1 : DÉCALAGE DE LA CARTE ---
    // Fenêtre = 740px, Map = 640px. UI = 100px.
    // Si on centre (0,0), la map va de -320 à +320.
    // L'UI va de -370 à -270. Il y a 50px de chevauchement.
    // On déplace tout le monde vers le HAUT de 50px.
    let vertical_shift = 50.0;

    let x_offset = -MAP_WIDTH / 2.0 + TILE_SIZE / 2.0;
    // On ajoute le vertical_shift ici
    let y_offset = (MAP_HEIGHT / 2.0 - TILE_SIZE / 2.0) + vertical_shift;

    // --- 2. Génération Visuelle de la Carte ---
    for (y, row) in level_data.iter().enumerate() {
        for (x, &tile_id) in row.iter().enumerate() {
            
            // Position dans le monde
            let pos = Vec2::new(
                x_offset + x as f32 * TILE_SIZE,
                y_offset - y as f32 * TILE_SIZE,
            );
            
            let tile_type = get_tile_type(tile_id);
            
            let mut rotation = Quat::IDENTITY; 
            let mut index = 0; 

            // Logique d'affichage (Match complet avec rotations corrigées)
            match tile_id {
                // --- Tuiles Composites (Eau + Sable/Terre) ---
                8 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(5, 0), Quat::IDENTITY),
                9 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(5, 0), Quat::from_rotation_z(-90.0f32.to_radians())),
                10 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(5, 0), Quat::from_rotation_z(180.0f32.to_radians())),
                11 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(5, 0), Quat::from_rotation_z(90.0f32.to_radians())),
                
                12 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(6, 0), Quat::IDENTITY),
                13 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(6, 0), Quat::from_rotation_z(-90.0f32.to_radians())),
                14 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(6, 0), Quat::from_rotation_z(180.0f32.to_radians())),
                15 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(6, 0), Quat::from_rotation_z(90.0f32.to_radians())),

                16 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(4, 0), Quat::IDENTITY),
                17 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(4, 0), Quat::from_rotation_z(-90.0f32.to_radians())),
                18 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(4, 0), Quat::from_rotation_z(180.0f32.to_radians())),
                19 => spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(0, 0), get_atlas_index(4, 0), Quat::from_rotation_z(90.0f32.to_radians())),

                // --- Tuiles Simples ---
                _ => {
                    (index, rotation) = match tile_id {
                        0 => (get_atlas_index(9, 0), Quat::IDENTITY),
                        1 => (get_atlas_index(0, 0), Quat::IDENTITY),
                        2 => (get_atlas_index(8, 0), Quat::IDENTITY),
                        3 => (get_atlas_index(8, 0), Quat::from_rotation_z(-90.0f32.to_radians())),
                        4 => (get_atlas_index(7, 0), Quat::IDENTITY),
                        5 => (get_atlas_index(7, 0), Quat::from_rotation_z(-90.0f32.to_radians())),
                        6 => (get_atlas_index(7, 0), Quat::from_rotation_z(180.0f32.to_radians())),
                        7 => (get_atlas_index(7, 0), Quat::from_rotation_z(90.0f32.to_radians())),

                        20 => { // START
                            spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(8, 0), get_atlas_index(7, 2), Quat::IDENTITY);
                            (999, Quat::IDENTITY) // 999 = Ne pas spawner de tuile simple supplémentaire
                        },
                        21 => { // END
                            spawn_composite_tile(&mut commands, &assets, pos, get_atlas_index(8, 0), get_atlas_index(8, 2), Quat::IDENTITY);
                            (999, Quat::IDENTITY)
                        },
                        _ => (get_atlas_index(0, 0), Quat::IDENTITY),
                    };

                    if index != 999 {
                        commands.spawn((
                            SpriteSheetBundle {
                                texture: assets.sprite_atlas.clone(),
                                atlas: TextureAtlas {
                                    layout: assets.sprite_atlas_layout.clone(),
                                    index,
                                },
                                transform: Transform {
                                    translation: pos.extend(0.0),
                                    rotation,
                                    ..default()
                                },
                                ..default()
                            },
                            GameTile { tile_type },
                            GameComponent,
                            Name::new(format!("Tile ({x},{y})")),
                        ));
                    }
                }
            }

            // On capture la position de départ pour le pathfinding
            if tile_type == TileType::Start {
                start_pos = pos;
            }
        }
    }

    // --- 3. Calcul du Chemin (Pathfinding) ---
    // On construit la liste des points que l'ennemi devra suivre
    
    let mut path_points = Vec::new();
    path_points.push(start_pos);

    // On convertit la position de départ (pixels) en coordonnées de grille (0-19)
    let mut grid_x = ((start_pos.x - x_offset) / TILE_SIZE).round() as i32;
    let mut grid_y = ((y_offset - start_pos.y) / TILE_SIZE).round() as i32;
    
    let mut last_grid_pos = (grid_x, grid_y); // Pour ne pas revenir en arrière

    // Boucle de recherche de chemin
    for _ in 0..100 { // Sécurité anti-boucle infinie
        let mut found_next = false;
        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Haut, Bas, Gauche, Droite

        for (dx, dy) in neighbors {
            let nx = grid_x + dx;
            let ny = grid_y + dy;

            // Vérifications limites + on évite le retour arrière
            if nx < 0 || ny < 0 || nx >= 20 || ny >= 20 { continue; }
            if (nx, ny) == last_grid_pos { continue; }

            let tile_id = level_data[ny as usize][nx as usize];
            let tile_type = get_tile_type(tile_id);

            // Si c'est une route ou la fin, c'est le bon chemin
            if tile_type == TileType::Road || tile_type == TileType::End {
                last_grid_pos = (grid_x, grid_y);
                grid_x = nx;
                grid_y = ny;

                let next_world_pos = Vec2::new(
                    x_offset + nx as f32 * TILE_SIZE,
                    y_offset - ny as f32 * TILE_SIZE,
                );
                path_points.push(next_world_pos);
                found_next = true;
                
                if tile_type == TileType::End {
                    break; // On a trouvé la fin, on sort du 'for neighbors'
                }
                break; // On a trouvé le prochain pas, on passe à l'itération suivante
            }
        }

        if !found_next { break; } // Plus de route trouvée ou cul de sac
        
        // Si on est sur la tuile de fin, on arrête tout
        let current_tile_id = level_data[grid_y as usize][grid_x as usize];
        if get_tile_type(current_tile_id) == TileType::End {
            break;
        }
    }

    println!("Chemin calculé avec succès : {} points", path_points.len());

    // On insère la Ressource pour que le système d'Ennemis puisse la lire
    commands.insert_resource(Path {
        points: path_points,
    });
}

fn setup_game_ui(mut commands: Commands, assets: Res<GameAssets>) {
    // --- CORRECTION 2 : COULEUR EXACTE ---
    // Java: new Color(220, 123, 15)
    let bar_color = Color::rgb_u8(220, 123, 15);

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: bar_color.into(),
            ..default()
        },
        GameComponent,
    ))
    .with_children(|parent| {
        spawn_tower_button(parent, &assets, TowerType::Canon);
        spawn_tower_button(parent, &assets, TowerType::Archer);
        spawn_tower_button(parent, &assets, TowerType::Wizard);
    });
}

fn spawn_tower_button(parent: &mut ChildBuilder, assets: &Res<GameAssets>, tower_type: TowerType) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(50.0), // Taille ajustée comme en Java
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)), // Espacement
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // --- CORRECTION 4 : Bordure ---
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            // Couleur de fond neutre (Java utilise GRAY)
            background_color: Color::GRAY.into(),
            // Couleur de bordure par défaut (Noir)
            border_color: BorderColor(Color::BLACK), 
            ..default()
        },
        TowerButton { tower_type },
    )).with_children(|parent| {
        // --- CORRECTION 3 : IMAGE ATLAS DANS L'UI ---
        // On utilise AtlasImageBundle pour afficher JUSTE le sprite
        parent.spawn(AtlasImageBundle {
            style: Style {
                width: Val::Percent(100.0),  // Remplir le bouton
                height: Val::Percent(100.0),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: assets.sprite_atlas_layout.clone(),
                index: tower_type.get_sprite_index(),
            },
            image: UiImage::new(assets.sprite_atlas.clone()),
            ..default()
        });
        // Pas de texte, comme demandé.
    });
}

fn spawn_composite_tile(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    pos: Vec2,
    base_index: usize,
    overlay_index: usize,
    overlay_rotation: Quat, // La rotation est maintenant corrigée
) {
    // Couche de base (Eau), z=0.0
    commands.spawn((
        SpriteSheetBundle {
            texture: assets.sprite_atlas.clone(),
            atlas: TextureAtlas {
                layout: assets.sprite_atlas_layout.clone(),
                index: base_index,
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
        GameTile { tile_type: TileType::Water }, 
        GameComponent,
        Name::new(format!("Tile ({},{}) - Base", pos.x, pos.y)),
    ));

    // Couche superposée (Sable), z=0.1
    commands.spawn((
        SpriteSheetBundle {
            texture: assets.sprite_atlas.clone(),
            atlas: TextureAtlas {
                layout: assets.sprite_atlas_layout.clone(),
                index: overlay_index,
            },
            transform: Transform {
                translation: pos.extend(0.1), // z=0.1
                rotation: overlay_rotation, // Applique la rotation (corrigée)
                ..default()
            },
            ..default()
        },
        GameTile { tile_type: TileType::Water }, 
        GameComponent,
        Name::new(format!("Tile ({},{}) - Overlay", pos.x, pos.y)),
    ));
}

// Gère le clic sur les boutons du bas
fn tower_button_interaction(
    // --- CORRECTION 5 : Bug de sélection ---
    // On enlève le filtre "Changed<Interaction>" pour que la logique
    // s'exécute à chaque frame. Cela garantit que si je clique sur B,
    // A perd instantanément son statut "Selected".
    mut interaction_query: Query<
        (&Interaction, &TowerButton, &mut BorderColor),
        With<Button>,
    >,
    mut selected_tower: ResMut<SelectedTower>,
) {
    for (interaction, tower_button, mut border_color) in interaction_query.iter_mut() {
        
        // Est-ce que ce bouton correspond à la tour sélectionnée ?
        let is_selected = selected_tower.tower_type == Some(tower_button.tower_type);

        match *interaction {
            Interaction::Pressed => {
                selected_tower.tower_type = Some(tower_button.tower_type);
                // Visuel immédiat : Bordure noire (comme sélectionné) ou spécifique clic
                *border_color = BorderColor(Color::BLACK);
            }
            Interaction::Hovered => {
                // Hover : Bordure blanche
                *border_color = BorderColor(Color::WHITE);
            }
            Interaction::None => {
                // Normal
                if is_selected {
                    // Si sélectionné : Bordure Noire (ou une autre couleur si vous préférez)
                    // En Java, il y a un carré autour.
                    *border_color = BorderColor(Color::BLACK); 
                } else {
                    // Pas sélectionné : Bordure transparente ou couleur du bouton
                    // Pour faire "propre", on peut mettre la même couleur que le fond (Gris)
                    *border_color = BorderColor(Color::GRAY);
                }
            }
        }
    }
}

// Gère le clic sur la grille pour poser une tour
fn grid_click_interaction(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_tower: Res<SelectedTower>,
    assets: Res<GameAssets>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Some(tower_type) = selected_tower.tower_type else { return; };
        let (camera, camera_transform) = camera_q.single();
        let Some(window) = windows.get_single().ok() else { return };

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Constantes (doivent être identiques à setup_game)
            const TILE_SIZE: f32 = 32.0;
            const MAP_WIDTH: f32 = 20.0 * TILE_SIZE;
            const MAP_HEIGHT: f32 = 20.0 * TILE_SIZE;
            let vertical_shift = 50.0; // Même décalage !
            let x_offset = -MAP_WIDTH / 2.0 + TILE_SIZE / 2.0;
            let y_offset = (MAP_HEIGHT / 2.0 - TILE_SIZE / 2.0) + vertical_shift;

            let grid_x = ((world_position.x - x_offset) / TILE_SIZE).round();
            let grid_y = ((y_offset - world_position.y) / TILE_SIZE).round();

            if grid_x >= 0.0 && grid_x < 20.0 && grid_y >= 0.0 && grid_y < 20.0 {
                // Vérifier l'UI : si la souris est trop basse, on ne clique pas
                // (L'UI prend les 100px du bas)
                if world_position.y < (-370.0 + 100.0) { return; } // -370 = bas de l'écran

                let ix = grid_x as usize;
                let iy = grid_y as usize;
                let level_data = level::get_level_data();
                let tile_id = level_data[iy][ix];

                if tile_id == 0 { // Herbe uniquement
                    let snap_pos = Vec2::new(x_offset + grid_x * TILE_SIZE, y_offset - grid_y * TILE_SIZE);
                    let (range, damage, cooldown) = tower_type.get_stats();

                    commands.spawn((
                        SpriteSheetBundle {
                            texture: assets.sprite_atlas.clone(),
                            atlas: TextureAtlas { layout: assets.sprite_atlas_layout.clone(), index: tower_type.get_sprite_index() },
                            transform: Transform::from_xyz(snap_pos.x, snap_pos.y, 2.0), 
                            ..default()
                        },
                        Tower { range, damage, cooldown: Timer::from_seconds(cooldown, TimerMode::Repeating) },
                        GameComponent,
                        Name::new(format!("Tower")),
                    ));
                }
            }
        }
    }
}

// Traduit la logique de TileManager.java
// et Constants.java
fn get_tile_type(tile_id: u32) -> TileType {
    match tile_id {
        // ID 0 = GRASS_TILE (type 1)
        0 => TileType::Grass,
        
        // IDs 2-7 = ROAD_TILE (type 2)
        2..=7 => TileType::Road,
        
        // ID 20 = START_TILE (type 3)
        20 => TileType::Start,
        
        // ID 21 = END_TILE (type 4)
        21 => TileType::End,
        
        // Tous les autres (1, 8-19) sont WATER_TILE (type 0)
        _ => TileType::Water,
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameComponent>>) {
    println!("Nettoyage du jeu (Playing)...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // On retire aussi la ressource Path et SelectedTower
    commands.remove_resource::<Path>();
    commands.remove_resource::<SelectedTower>();
}

fn tower_shooting(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut tower_query: Query<(&Transform, &mut Tower)>, // Les tours
    enemy_query: Query<(Entity, &Transform), With<Enemy>>, // Les ennemis
) {
    for (tower_transform, mut tower) in tower_query.iter_mut() {
        // Avancer le cooldown de la tour
        tower.cooldown.tick(time.delta());

        // Si la tour est prête à tirer
        if tower.cooldown.just_finished() {

            // Trouver l'ennemi le plus proche à portée
            let mut closest_enemy: Option<Entity> = None;
            let mut min_dist_sq = tower.range * tower.range; // Comparaison au carré pour perf

            let tower_pos = tower_transform.translation.truncate();

            for (enemy_entity, enemy_transform) in enemy_query.iter() {
                let enemy_pos = enemy_transform.translation.truncate();
                let dist_sq = tower_pos.distance_squared(enemy_pos);

                if dist_sq <= min_dist_sq {
                    min_dist_sq = dist_sq;
                    closest_enemy = Some(enemy_entity);
                }
            }

            // Si on a trouvé une cible, FEU !
            if let Some(target) = closest_enemy {
                // CORRECTION : 17 est la flèche (Ligne 2, colonne 8)
                // 27 était le "S" du départ.
                let projectile_sprite_index = 17;

                commands.spawn((
                    SpriteSheetBundle {
                        texture: assets.sprite_atlas.clone(),
                        atlas: TextureAtlas {
                            layout: assets.sprite_atlas_layout.clone(),
                            index: projectile_sprite_index,
                        },
                        transform: Transform::from_xyz(tower_pos.x, tower_pos.y, 2.0), // Même hauteur que la tour
                        ..default()
                    },
                    Projectile {
                        target,
                        damage: tower.damage,
                        speed: 300.0, // Rapide !
                    },
                    GameComponent, // Pour le nettoyage
                ));

                // Réinitialiser le cooldown
                tower.cooldown.reset();
            }
        }
    }
}