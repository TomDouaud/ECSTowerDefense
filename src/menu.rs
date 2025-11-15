use bevy::prelude::*;
use crate::AppState;

// équivalent de la classe "Menu.java"
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ajouts des systemes qui s'éxécutent lors des changements d'état
            
            // S'exécute 1x quand on *entre* dans AppState::Menu
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            
            // S'exécute 1x quand on *sort* de AppState::Menu
            .add_systems(OnExit(AppState::Menu), cleanup_menu);
    }
}

// équivalent du constructeur Menu() ou initButtons() en java
fn setup_menu() {
    println!("Bienvenue au Menu !");
    // TODO Créer les boutons et l'UI du menu ici
}

// nettoyage du menu quand on en sort
fn cleanup_menu() {
    println!("Nettoyage du Menu...");
}