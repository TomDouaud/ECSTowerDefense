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
    pub start: Vec2, // Coordonnées "monde" (pas en grille)
    pub end: Vec2,
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

    let start_point = level::get_start_point();
    let end_point = level::get_end_point();
    
    let mut start_pos = Vec2::ZERO;
    let mut end_pos = Vec2::ZERO;

    const TILE_SIZE: f32 = 32.0;
    const MAP_WIDTH: f32 = 20.0 * TILE_SIZE;
    const MAP_HEIGHT: f32 = 20.0 * TILE_SIZE;
    let x_offset = -MAP_WIDTH / 2.0 + TILE_SIZE / 2.0;
    let y_offset = MAP_HEIGHT / 2.0 - TILE_SIZE / 2.0;
    
    let level_data = level::get_level_data();

    // Boucle de rendu de la carte
    for (y, row) in level_data.iter().enumerate() {
        for (x, &tile_id) in row.iter().enumerate() {
            
            let pos = Vec2::new(
                x_offset + x as f32 * TILE_SIZE,
                y_offset - y as f32 * TILE_SIZE,
            );
            
            let tile_type = get_tile_type(tile_id); // Pour la logique de jeu
            
            let mut rotation = Quat::IDENTITY; 
            let mut index = 0; 
            
            match tile_id {
                // ID 8-19 : Tuiles composites (Eau + Sable)
                8 => { // BL_WATER_CORNER
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(5, 0), 
                                         Quat::IDENTITY);
                },
                9 => { // TL_WATER_CORNER: (0,0) + (5,0) rot 90
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(5, 0), 
                                         Quat::from_rotation_z(-90.0f32.to_radians())); // <-- CORRIGÉ
                },
                10 => { // TR_WATER_CORNER: (0,0) + (5,0) rot 180
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(5, 0), 
                                         Quat::from_rotation_z(180.0f32.to_radians())); // Inchangé
                },
                11 => { // BR_WATER_CORNER: (0,0) + (5,0) rot 270
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(5, 0), 
                                         Quat::from_rotation_z(90.0f32.to_radians())); // <-- CORRIGÉ (était 270)
                },
                12 => { // T_WATER (Plage haut)
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(6, 0), 
                                         Quat::IDENTITY);
                },
                13 => { // R_WATER (Plage droite)
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(6, 0), 
                                         Quat::from_rotation_z(-90.0f32.to_radians())); // <-- CORRIGÉ
                },
                14 => { // B_WATER (Plage bas)
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(6, 0), 
                                         Quat::from_rotation_z(180.0f32.to_radians())); // Inchangé
                },
                15 => { // L_WATER (Plage gauche)
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(6, 0), 
                                         Quat::from_rotation_z(90.0f32.to_radians())); // <-- CORRIGÉ (était 270)
                },
                 // ID 16-19: Îles
                16 => { // TL_ISLE
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(4, 0), 
                                         Quat::IDENTITY);
                },
                17 => { // TR_ISLE
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(4, 0), 
                                         Quat::from_rotation_z(-90.0f32.to_radians())); // <-- CORRIGÉ
                },
                18 => { // BR_ISLE
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(4, 0), 
                                         Quat::from_rotation_z(180.0f32.to_radians())); // Inchangé
                },
                19 => { // BL_ISLE
                    spawn_composite_tile(&mut commands, &assets, pos, 
                                         get_atlas_index(0, 0), 
                                         get_atlas_index(4, 0), 
                                         Quat::from_rotation_z(90.0f32.to_radians())); // <-- CORRIGÉ (était 270)
                },
                
                // --- Tuiles simples ---
                _ => {
                    (index, rotation) = match tile_id {
                        0 => (get_atlas_index(9, 0), Quat::IDENTITY),
                        1 => (get_atlas_index(0, 0), Quat::IDENTITY),
                        2 => (get_atlas_index(8, 0), Quat::IDENTITY),
                        3 => (get_atlas_index(8, 0), Quat::from_rotation_z(-90.0f32.to_radians())), // <-- CORRIGÉ
                        4 => (get_atlas_index(7, 0), Quat::IDENTITY),
                        5 => (get_atlas_index(7, 0), Quat::from_rotation_z(-90.0f32.to_radians())), // <-- CORRIGÉ
                        6 => (get_atlas_index(7, 0), Quat::from_rotation_z(180.0f32.to_radians())), // Inchangé
                        7 => (get_atlas_index(7, 0), Quat::from_rotation_z(90.0f32.to_radians())), // <-- CORRIGÉ (était 270)
                        
                        20 => { // START_PATH
                            spawn_composite_tile(&mut commands, &assets, pos, 
                                                 get_atlas_index(8, 0), 
                                                 get_atlas_index(7, 2), 
                                                 Quat::IDENTITY);
                            (999, Quat::IDENTITY) 
                        },
                        21 => { // END_PATH
                            spawn_composite_tile(&mut commands, &assets, pos, 
                                                 get_atlas_index(8, 0), 
                                                 get_atlas_index(8, 2), 
                                                 Quat::IDENTITY);
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
                            Name::new(format!("Tile ({x},{y}) - ID {tile_id}")),
                        ));
                    }
                }
            } // Fin du match tile_id
            
            if tile_type == TileType::Start {
                start_pos = pos;
            } else if tile_type == TileType::End {
                end_pos = pos;
            }
        }
    }
    
    commands.insert_resource(Path {
        start: start_pos,
        end: end_pos,
    });
    
    println!("Carte générée (rotations corrigées) ! Départ à: {start_pos:?}, Fin à: {end_pos:?}");
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