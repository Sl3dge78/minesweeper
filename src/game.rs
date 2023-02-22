
use crate::{math::*, renderer::*, input::*};

pub struct GameState {
    pub delta_time: f32,
    grid: Grid,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            delta_time : 0.0,
            grid: Default::default()
        }
    }
}

impl GameState {
    pub fn update(&mut self, _input: &Input) {

    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_2d();
        renderer.default_texture();
        self.grid.draw(renderer);
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
    pub fn draw(&self, renderer: &mut Renderer) {
        let height = 16;
        let width = 16;

        for y in 0..16 {
            for x in 0..16 {
                renderer.push_2d_quad((x * (width + 1)) as f32, (y * (height + 1)) as f32, width as f32, height as f32, Vec4::new(1.0, 1.0, 1.0, 1.0));
            }
        }
    }
}
