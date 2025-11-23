use bevy::prelude::*;
use crate::{AppState, GameAssets, game::Path, constants::enemies as EnemyConstants, game::PlayerStats}; 

// Le component Ennemi (juste avec une vitesse)
#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

// Le component Santé (points de vie actuels et max)
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct HealthBar;

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
            .insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(1.5, TimerMode::Repeating), // Un peu plus rapide
            })
            .add_systems(Update, 
                (spawn_enemies, move_enemies, animate_enemy_rotation, enemy_death_system, update_health_bars)
                .run_if(in_state(AppState::Playing).or_else(in_state(AppState::Simulation)))
            );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    path: Res<Path>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        if path.points.is_empty() { return; }
        
        let start_pos = path.points[0]; 
        let hp = 85; 
        let speed = 0.5 * 100.0; 

        // On spawn l'ennemi
        commands.spawn((
            SpriteSheetBundle {
                texture: assets.sprite_atlas.clone(),
                atlas: TextureAtlas {
                    layout: assets.sprite_atlas_layout.clone(),
                    index: 10, // Orc
                },
                transform: Transform::from_xyz(start_pos.x, start_pos.y, 1.0), 
                ..default()
            },
            Enemy { speed },
            Health { current: hp, max: hp },
            PathFollower { path_index: 1 }, 
            Name::new("Orc"),
        ))
        // ON AJOUTE DES ENFANTS (CHILDREN) À L'ENTITÉ
        .with_children(|parent| {
            // 1. Fond de la barre (Noir, un peu plus grand)
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(22.0, 6.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 20.0, 0.1), // Au dessus de la tête
                ..default()
            });

            // 2. Barre de vie (Rouge)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(20.0, 4.0)), // Taille max
                        ..default()
                    },
                    // On décale légèrement en z (0.2) pour être devant le noir
                    transform: Transform::from_xyz(0.0, 20.0, 0.2), 
                    ..default()
                },
                HealthBar, // Marqueur pour la mise à jour
            ));
        });
    }
}

fn move_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Enemy, &mut PathFollower)>,
    path: Res<Path>,
    time: Res<Time>,
    mut stats: ResMut<PlayerStats>, // <--- Ajoutez ceci
) {
    if path.points.is_empty() { return; }
    for (entity, mut transform, enemy, mut follower) in query.iter_mut() {
        if follower.path_index >= path.points.len() {
            stats.lives -= 1; // Perte de vie
            commands.entity(entity).despawn_recursive();
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
    }
}

fn animate_enemy_rotation(
    mut query: Query<(&mut Transform, &PathFollower)>,
    path: Res<Path>,
) {
    if path.points.is_empty() { return; }

    for (mut transform, follower) in query.iter_mut() {
        if follower.path_index < path.points.len() {
            let target = path.points[follower.path_index];
            let current = transform.translation.truncate();
            let diff = target - current;

            if diff.x.abs() > diff.y.abs() {
                // Note: La rotation affecte aussi la barre de vie !
                // Pour éviter que la barre tourne, il faudrait contrer la rotation
                // ou gérer le sprite séparément. Pour l'instant, simple rotation Y (miroir).
                if diff.x > 0.0 { transform.rotation = Quat::IDENTITY; } 
                else { transform.rotation = Quat::from_rotation_y(std::f32::consts::PI); }
            } 
        }
    }
}

fn update_health_bars(
    // On cherche les entités qui sont des barres de vie et qui ont un Parent
    mut bar_query: Query<(&mut Transform, &Parent), With<HealthBar>>,
    // On cherche la santé des Parents (les ennemis)
    health_query: Query<&Health>,
) {
    for (mut transform, parent) in bar_query.iter_mut() {
        // On récupère la santé du parent via l'Entity stockée dans 'Parent'
        if let Ok(health) = health_query.get(parent.get()) {
            // Calcul du pourcentage
            let percent = health.current as f32 / health.max as f32;
            // On réduit l'échelle sur X (largeur)
            // clamp pour éviter les valeurs négatives ou > 1
            transform.scale.x = percent.clamp(0.0, 1.0);
        }
    }
}

fn enemy_death_system(
    mut commands: Commands, 
    query: Query<(Entity, &Health)>,
    mut stats: ResMut<PlayerStats>, // <--- Ajoutez ceci
) {
    for (entity, health) in query.iter() {
        if health.current <= 0 {
            // Gain d'argent (ex: 5 gold par orc)
            // Idéalement, la récompense serait dans le composant Enemy
            stats.money += 5; 
            commands.entity(entity).despawn_recursive();
        }
    }
}