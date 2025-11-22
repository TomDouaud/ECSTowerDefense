use bevy::prelude::*;

// Les 3 types de tours dans le jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum TowerType {
    Canon = 0,
    Archer = 1,
    Wizard = 2,
}

// Composant principal d'une tour (sa portée, ses dégâts, son cooldown)
#[derive(Component)]
pub struct Tower {
    pub range: f32,
    pub damage: i32,
    pub cooldown: Timer,
}

// Données statiques (équivalent de Constants.Towers)
impl TowerType {
    pub fn get_stats(&self) -> (f32, i32, f32) {
        // Retourne (Range, Damage, Cooldown_Speed)
        match self {
            TowerType::Canon => (75.0, 15, 1.2), // Canon: Portée moyenne, gros dégâts, lent
            TowerType::Archer => (120.0, 5, 0.35), // Archer: Longue portée, dégâts faibles, rapide
            TowerType::Wizard => (100.0, 30, 2.0), // Wizard: Portée moyenne, très gros dégâts, très lent
        }
    }

    pub fn get_sprite_index(&self) -> usize {
        // Canon = (4, 1) -> index 14
        // Archer = (5, 1) -> index 15
        // Wizard = (6, 1) -> index 16
        match self {
            TowerType::Canon => 14,
            TowerType::Archer => 15,
            TowerType::Wizard => 16,
        }
    }
}