
use sdl2::event::Event;

use crate::{math::*, renderer::*, input::*};

pub struct GameState {
    pub delta_time: f32,
    grid: Grid,
    cell_size: i32,
    cell_offset: Vec2,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            delta_time : 0.0,
            grid: Default::default(),
            cell_size : 16,
            cell_offset : Vec2::new(0.0, 0.0),
        }
    }
}

impl GameState {
    pub fn event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => self.on_click(x, y),
                    _ => {}
                }
            }
            _ => {}
        }

    }

    pub fn update(&mut self, _input: &Input) {

    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_2d();
        renderer.default_texture();
        self.grid.draw(renderer, self.cell_size, self.cell_offset);
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        let x = ((x as f32 - self.cell_offset.x) / (self.cell_size as f32)) as i32; 
        let y = ((y as f32 - self.cell_offset.y) / (self.cell_size as f32)) as i32;
        println!("Clicked on {}, {}", x, y);

    }
}

#[derive(Copy, Clone)]
struct Cell {

}

impl Default for Cell {
    fn default() -> Self {
        Cell {}
    }
}

struct Grid {
    elems : [[Cell ; 16] ; 16],
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            elems: [[Cell {}; 16]; 16]
        }
    }
}

impl Grid {
    pub fn draw(&self, renderer: &mut Renderer, size: i32, offset: Vec2) {
        for y in 0..16 {
            for x in 0..16 {
                renderer.push_2d_quad(offset.x + (x * size) as f32, offset.y + (y * size) as f32, size as f32, size as f32, Vec4::new(1.0, 1.0, 1.0, 1.0));
            }
        }
    }
}
