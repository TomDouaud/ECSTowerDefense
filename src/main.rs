use bevy::prelude::*;

// enum de tous les états possibles de l'application (basé sur GameState.java)
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default] // L'état par défaut au lancement
    Menu,
    Playing,
    Settings,
}


pub mod menu;
pub mod game;
pub mod settings;

use menu::MenuPlugin;
use game::GamePlugin;
use settings::SettingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tower Defense (Bevy)".into(),
                // Taille récupérée sur GameScreen.java
                resolution: (640, 740).into(), 
                resizable: false,
                ..default()
            }),
            ..default()
        }))

        // Utilise notre systeme d'état AppState
        .init_state::<AppState>()

        // Ajout des scenes (vu comme des "plugins" Bevy)
        .add_plugins((
            MenuPlugin,
            GamePlugin,
            SettingsPlugin,
        ))

        // systeme de démarrage pour créer la caméra
        .add_systems(Startup, setup_camera)
        .run();
}

/// Lancement de la caméra 2D dans le monde
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}