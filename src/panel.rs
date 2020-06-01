use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::Display;
use std::path::Path;

use crate::sprite::Sprite;

#[derive(Copy, Clone)]
pub enum PanelKind {
    Circle = 0,
    Diamond,
    Heart,
    Star,
    Up,
    Down,
    Stone
}

impl Display for PanelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PanelKind::Circle => "circle",
            PanelKind::Diamond => "diamond",
            PanelKind::Heart => "heart",
            PanelKind::Star => "star",
            PanelKind::Up => "up",
            PanelKind::Down => "down",
            PanelKind::Stone => "stone",
        })
    }
}

// TODO: is there a better way to get a random tile?
impl Distribution<PanelKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PanelKind {
        match rng.gen_range(0, 7) {
            0 => PanelKind::Circle,
            1 => PanelKind::Diamond,
            2 => PanelKind::Heart,
            3 => PanelKind::Star,
            4 => PanelKind::Up,
            5 => PanelKind::Down,
            _ => PanelKind::Stone,
        }
    }
}

/// Current state of a panel's animation
pub enum PanelAnimState {
    Normal,
    Dark,
    Bounce,
    Clear
}

impl Display for PanelAnimState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PanelAnimState::Normal => "",
            PanelAnimState::Dark => "Dark",
            PanelAnimState::Bounce => "Bounce",
            PanelAnimState::Clear => "Clear",
        })
    }
}

pub struct Panel {
    kind: PanelKind,
    sprite: Sprite
}

impl Panel {
    pub fn new(kind: PanelKind) -> Self {
        let mut res = Self {kind, sprite: Sprite::new(Path::new("res/sprites/panels.json"))};
        res.sprite.set_animation(res.kind.to_string());
        res
    }

    pub fn new_with_random_kind() -> Self {
        Self::new(rand::random())
    }

    pub fn mut_sprite(&mut self) -> &mut Sprite {
        &mut self.sprite
    }

    /// Convenience wrapper for sprite.[set_animation](../sprite/struct.Sprite.html)
    /// which sets the sprite's animation based off of what kind of panel this is
    pub fn set_anim_state(&mut self, state: PanelAnimState) {
        self.sprite.set_animation(format!("{}{}", self.kind, state));
    }

    pub fn update(&mut self, delta_time: f64) {
        self.sprite.update(delta_time);
    }
}
