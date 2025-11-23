use bevy::prelude::*;
use crate::{AppState, enemy::Enemy}; // On aura besoin de checker si la cible est un ennemi

// Composant Projectile
#[derive(Component)]
pub struct Projectile {
    pub target: Entity, // L'entité ennemie visée
    pub damage: i32,
    pub speed: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_projectiles.run_if(in_state(AppState::Playing).or_else(in_state(AppState::Simulation))));
    }
}

fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    // On a besoin de la position des ennemis pour savoir où aller
    enemy_query: Query<&GlobalTransform, With<Enemy>>, 
    time: Res<Time>,
    // On a besoin d'accéder à la santé des ennemis pour faire des dégâts
    mut enemy_health_query: Query<&mut crate::enemy::Health>,
) {
    for (proj_entity, mut proj_transform, projectile) in projectile_query.iter_mut() {
        
        // 1. Est-ce que la cible existe toujours ?
        if let Ok(target_transform) = enemy_query.get(projectile.target) {
            
            // 2. Calculer la direction
            let target_pos = target_transform.translation().truncate();
            let current_pos = proj_transform.translation.truncate();
            let direction = target_pos - current_pos;
            let distance = direction.length();
            
            let step = projectile.speed * time.delta_seconds();

            // 3. Si on touche la cible (ou qu'on la dépasse)
            if distance <= step {
                // Appliquer les dégâts
                if let Ok(mut health) = enemy_health_query.get_mut(projectile.target) {
                    health.current -= projectile.damage;
                }
                
                // Détruire le projectile
                commands.entity(proj_entity).despawn();
            } else {
                // Sinon, avancer
                let movement = direction.normalize() * step;
                proj_transform.translation.x += movement.x;
                proj_transform.translation.y += movement.y;
                
                // Rotation du projectile vers la cible (optionnel mais joli)
                // Angle en radians
                let angle = direction.y.atan2(direction.x);
                proj_transform.rotation = Quat::from_rotation_z(angle);
            }

        } else {
            // La cible est morte avant que le projectile n'arrive
            commands.entity(proj_entity).despawn();
        }
    }
}