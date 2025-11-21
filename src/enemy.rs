use bevy::prelude::*;
use crate::{AppState, GameAssets, game::Path}; 

// Le component Ennemi (juste avec une vitesse)
#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

// Le path finding marche en suivant a chaque fois le point suivant
// sur un chemin prédéfini (Path). On garde l'index du point actuel.
#[derive(Component)]
pub struct PathFollower {
    pub path_index: usize,
}

// gère le temps entre les apparitions (Spawning)
#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // le timer se répète toutes les 2 secondes
            .insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            })
            .add_systems(Update, 
                (spawn_enemies, move_enemies, animate_enemy_rotation)
                .run_if(in_state(AppState::Playing)) // Ne tourne que pendant le jeu
            );
    }
}


fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    path: Res<Path>, // récupèration du chemin complet
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        // Position de départ (le point 0 du chemin, en pixels)
        let start_pos = path.points[0]; 

        // On spawn un "Orc" (sprite index 0 dans la ligne des ennemis ?)
        // Dans votre atlas, les ennemis sont à la ligne 1 (y=32px)
        // Orc = 0, Bat = 1, etc.
        
        commands.spawn((
            SpriteSheetBundle {
                texture: assets.sprite_atlas.clone(),
                atlas: TextureAtlas {
                    layout: assets.sprite_atlas_layout.clone(),
                    index: 10, // 10 = Premier sprite de la 2ème ligne (Orc)
                },
                transform: Transform::from_xyz(start_pos.x, start_pos.y, 1.0), // Pour afficher au-dessus du fond
                ..default()
            },
            Enemy { speed: 60.0 }, // Vitesse en pixels/seconde
            PathFollower { path_index: 1 }, // vise le point 1 (le 0 est le départ)
            Name::new("Orc"),
        ));
    }
}

fn move_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Enemy, &mut PathFollower)>,
    path: Res<Path>,
    time: Res<Time>,
) {
    for (entity, mut transform, enemy, mut follower) in query.iter_mut() {
        // Vérifier si l'ennemi a atteint la fin du chemin
        if follower.path_index >= path.points.len() {
            println!("Ennemi arrivé à la fin !");
            commands.entity(entity).despawn();
            continue;
        }

        // Essayer de trouver la cible actuelle
        let target = path.points[follower.path_index];
        
        // Calcule le vecteur de direction (Cible - Position Actuelle)
        let direction = target - transform.translation.truncate(); // truncate passe de Vec3 à Vec2
        let distance = direction.length();

        // Calcule le déplacement pour cette frame
        let step = enemy.speed * time.delta_seconds();

        // Logique de déplacement
        if distance <= step {
            // Si l'énnemi est assez proche pour atteindre le point cette frame :
            // Il se "téléporte" sur le point exact
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            
            // Et il passe au point suivant
            follower.path_index += 1;
        } else {
            // Sinon, l'ennemi avance vers la cible
            // normalize() rend le vecteur de longueur 1, et il faut multiplier par la vitesse
            let movement = direction.normalize() * step;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

fn animate_enemy_rotation(
    mut query: Query<(&mut Transform, &PathFollower)>,
    path: Res<Path>,
) {
    for (mut transform, follower) in query.iter_mut() {
        if follower.path_index < path.points.len() {
            let target = path.points[follower.path_index];
            let current = transform.translation.truncate();
            let diff = target - current;

            if diff.x.abs() > diff.y.abs() {
                if diff.x > 0.0 { transform.rotation = Quat::IDENTITY; } // Droite
                else { transform.rotation = Quat::from_rotation_y(std::f32::consts::PI); } // Gauche (miroir)
            } 
        }
    }
}