use bevy::prelude::*;
use crate::AppState; 

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

// équivalent constructeur Playing()
fn setup_game() {
    println!("Lancement du jeu (Playing) !");
    // Lire le level build et la charger la carte ici
}

fn cleanup_game() {
    println!("Nettoyage du jeu (Playing)...");
    // destruction des entités du jeu ici
}