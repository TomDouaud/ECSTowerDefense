use bevy::prelude::*;
use crate::AppState; 

// Équivalent de "Settings.java"
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Settings), setup_settings)
            .add_systems(OnExit(AppState::Settings), cleanup_settings);
    }
}

fn setup_settings() {
    println!("Ouverture des Paramètres !");
}

fn cleanup_settings() {
    println!("Nettoyage des Paramètres...");
}