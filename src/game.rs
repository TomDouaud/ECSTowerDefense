use bevy::{prelude::*, sprite::SpriteSheetBundle};
use crate::{
    AppState, 
    GameAssets,
    level,
    constants::tiles as TileTypes, // les constantes de tuiles
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

// Équivalent de "Playing.java"
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systèmes pour l'état "Playing"
            .add_systems(OnEnter(AppState::Playing), setup_game)
            .add_systems(OnExit(AppState::Playing), cleanup_game);
            
            // TODO ajouter les systèmes de mise à jour ici
            // .add_systems(Update, (enemy_movement, tower_attack)
            //     .run_if(in_state(AppState::Playing)));
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
    
    // Calcul des offsets pour centrer la carte (0,0 au centre)
    let x_offset = -MAP_WIDTH / 2.0 + TILE_SIZE / 2.0;
    let y_offset = MAP_HEIGHT / 2.0 - TILE_SIZE / 2.0;

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
    // On retire aussi la ressource Path
    commands.remove_resource::<Path>();
}