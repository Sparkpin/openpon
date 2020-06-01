use sdl2::render::WindowCanvas;
use std::collections::VecDeque;
use std::convert::TryFrom;
use arrayvec::ArrayVec;

use crate::panel::{Panel, PanelAnimState, PANEL_HEIGHT, PANEL_WIDTH};
use crate::texture_manager::TextureManager;
use crate::vec2::Vec2;

const FIELD_WIDTH: usize = 6;
const FIELD_HEIGHT: usize = 13; // 12 + row below screen
const BASE_STACK_RECIPROCAL_SPEED: f64 = 0.4; // seconds per pixel
const SPEEDUP_PER_LEVEL: f64 = 0.045;
const MAX_STACK_RECIPROCAL_SPEED: f64 = 1.0 / 60.0;

pub struct Playfield {
    panels: VecDeque<[Option<Panel>; FIELD_WIDTH]>,
    pos: Vec2<i32>,
    timer: f64,
    level: i8,
    stack_offset: i32,
    stack_stopped: bool
}

impl Playfield {
    pub fn new(x: i32, y: i32) -> Self {
        let mut res = Self {
            panels: VecDeque::with_capacity(FIELD_HEIGHT),
            pos: Vec2::new(x, y),
            timer: 0.0,
            level: 0,
            stack_offset: 0,
            stack_stopped: false
        };
        for _ in 0..4 {
            res.panels.push_front(res.generate_row());
            res.illuminate_bottom_row();
        }
        res.panels.push_front(res.generate_row());
        res
    }

    pub fn set_stack_stopped(&mut self, stopped: bool) {
        self.stack_stopped = stopped;
    }

    fn stack_raise_rate(&self) -> f64 {
        (BASE_STACK_RECIPROCAL_SPEED - SPEEDUP_PER_LEVEL * self.level as f64)
            .max(MAX_STACK_RECIPROCAL_SPEED)
    }

    /// Map a function over all elements in the playfield.
    /// Takes effect immediately and does not collect into a Vec.
    pub fn unlazy_map(&mut self, func: &mut impl FnMut(&mut Panel)) {
        for row in &mut self.panels {
            for panel in row {
                if let Some(panel) = panel {
                    func(panel);
                }
            }
        }
    }

    fn generate_row(&self) -> [Option<Panel>; FIELD_WIDTH] {
        let mut res = ArrayVec::new();
        for _ in 0..FIELD_WIDTH {
            let mut panel = Panel::new_with_random_kind();
            panel.set_anim_state(PanelAnimState::Dark);
            res.push(Some(panel));
        }
        match res.into_inner() {
            Ok(arr) => arr,
            Err(e) => panic!(e)
        }
    }

    /// Change the bottom row from dark panels to light panels
    fn illuminate_bottom_row(&mut self) {
        let bottom_row = self.panels.get_mut(0);
        if bottom_row.is_none() {
            return;
        }

        for panel in bottom_row.unwrap() {
            if let Some(panel) = panel {
                panel.set_anim_state(PanelAnimState::Normal);
            }
        }
    }

    fn raise_stack_one_pixel(&mut self) {
        self.stack_offset += 1;
        if self.stack_offset >= PANEL_HEIGHT {
            self.stack_offset = 0;
            self.illuminate_bottom_row();
            while self.panels.len() >= FIELD_HEIGHT - 1 {
                let row = self.panels.pop_back();
                if let Some(row) = row {
                    if row.iter().any(|x| x.is_some()) {
                        // You lost :(
                        todo!("Lose");
                    }
                }
            }
            self.panels.push_front(self.generate_row());
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.stack_stopped {
            return;
        }
        self.timer += delta_time;
        let current_raise_rate = self.stack_raise_rate();
        while self.timer >= current_raise_rate {
            self.timer -= current_raise_rate;
            self.raise_stack_one_pixel();
        }
    }

    pub fn draw<T>(&mut self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager<T>) {
        for y in 0..FIELD_HEIGHT {
            let row = self.panels.get_mut(y);
            if row.is_none() {
                continue;
            }
            let row = row.unwrap();

            for x in 0..FIELD_WIDTH {
                let maybe_panel = row.get_mut(x);
                if maybe_panel.is_none() {
                    continue;
                }
                let maybe_panel = maybe_panel.unwrap();
                if let Some(panel) = maybe_panel {
                    let panel_x = i32::try_from(x).unwrap() * PANEL_WIDTH;
                    let panel_y = i32::try_from(y).unwrap() * PANEL_HEIGHT;
                    let offset = self.pos.clone() + (panel_x, -panel_y - self.stack_offset).into();
                    panel.mut_sprite().draw_with_offset(offset, canvas, texture_manager);
                }
            }
        }
    }

}
