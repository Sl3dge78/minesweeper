
use sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton};

use crate::{math::*, renderer::*, input::*, sprite_sheet::SpriteSheet, resources::*};

enum State {
    Playing,
    Lost,
}

pub struct GameState {
    pub delta_time: f32,
    grid: Grid,
    state: State,
    camera_position: Vec2i,
}

const SPRITE_HIDDEN: (i32, i32) = (0, 2);
const SPRITE_FLAG: (i32, i32) = (1, 2);
const SPRITE_MINE: (i32, i32) = (2, 2);
const SPRITE_0: (i32, i32) = (3, 2);

const DENSITY: f32 = 0.1;

impl Default for GameState {
    fn default() -> Self {
        GameState {
            delta_time : 0.0,
            grid: Grid::new(),
            state: State::Playing,
            camera_position: Vec2i::new(0, 0),
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            grid: Grid::new(),
            ..Default::default()
        }
    }

    pub fn event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if let State::Playing = self.state {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => self.on_left_click(x, y),
                        sdl2::mouse::MouseButton::Right => self.on_right_click(x, y),
                        _ => {}
                    }
                }
            },
            Event::KeyDown { scancode, .. } => {
                if let Some(scancode) = scancode {
                    match scancode {
                        Scancode::R => { 
                            self.grid = Grid::new();
                            self.state = State::Playing;
                        },
                        Scancode::M => {
                            self.grid.show_all_mines();
                        },
                        _ => {},
                    }   
                }
            }
            _ => {}
        }

    }

    pub fn update(&mut self, input: &Input) {
        if input.mouse.is_mouse_button_pressed(MouseButton::Middle) {
            self.camera_position += Vec2i::new(- input.rel_mouse.x(), - input.rel_mouse.y());
        }

    }

    pub fn draw(&self, renderer: &mut Renderer, resources: &Resources) {
        Renderer::clear(Vec4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0});
        renderer.begin_2d();
        renderer.default_texture();
        
        self.grid.draw(renderer, resources.get("./res/sprites.png").as_texture(), self.camera_position);
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        return self.camera_position.vec2() + pos;
    }

    pub fn on_left_click(&mut self, x: i32, y: i32) {
        let pos = self.screen_to_world(Vec2::new(x as f32, y as f32));
        if self.grid.reveal(pos.vec2i()) {
            self.loose();
        }
    }

    pub fn on_right_click(&mut self, x: i32, y: i32) {
        let pos = self.screen_to_world(Vec2::new(x as f32, y as f32));
        self.grid.flag(pos.vec2i());
    }

    fn loose(&mut self) {
        println!("You loose !");
        self.state = State::Lost;
        self.grid.show_all_mines();
    }
}

#[derive(Copy, Clone)]
enum CellContents {
    Empty(i32),
    Mine
}

#[derive(Copy, Clone)]
struct Cell {
    revealed: bool,
    flag: bool,
    contents: CellContents,
}

impl Default for Cell {
    fn default() -> Self {
        Self { revealed: false, flag:false, contents: CellContents::Empty(0) }
    }
}

const CELL_SIZE: u32 = 16;
const CHUNK_SIZE: u32 = 16;
const CHUNK_LEN: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;

struct Chunk {
    elems : [Cell;(CHUNK_SIZE * CHUNK_SIZE) as usize],
    position: Vec2i,
}

impl Chunk {
    pub fn new(position: Vec2i, density: f32) -> Chunk {
        let mut result = Chunk {
            elems: [Default::default();256],
            position,
        };
        let nb_mines: u32 = (density * (CHUNK_LEN) as f32) as u32;
        for _ in 0..nb_mines {
            loop {
                let x = rand::random::<u32>() % CHUNK_SIZE;
                let y = rand::random::<u32>() % CHUNK_SIZE;
                let index: usize = (x + y * CHUNK_SIZE) as usize;
                match result.elems[index].contents {
                    CellContents::Mine => continue,
                    _ => { 
                        result.place_mine(x, y);
                        break;
                    },
                };
            }
        }
        result
    }

    fn idx(x: u32, y: u32) -> usize {
        (x + y * CHUNK_SIZE) as usize
    }

    fn place_mine(&mut self, x: u32, y: u32) {
        let idx = Chunk::idx(x, y);
        self.elems[idx].contents = CellContents::Mine;
        let start_x = if x == 0 { 0 } else { x - 1 };
        let end_x = if x == CHUNK_SIZE - 1 { CHUNK_SIZE - 1 } else { x + 1 };

        let start_y = if y == 0 { 0 } else { y - 1};
        let end_y = if y == CHUNK_SIZE - 1 { CHUNK_SIZE - 1 } else { y + 1};
        for x2 in start_x..=end_x {
            for y2 in start_y..=end_y {
                if x2 == x && y2 == y { continue; }
                let idx = Chunk::idx(x2, y2);
                if let CellContents::Empty(ref mut nb) = self.elems[idx].contents {
                    *nb += 1;
                }
            }
        }
    }

    pub fn show_all_mines(&mut self) {
        for mut c in &mut self.elems {
            if let CellContents::Mine = c.contents {
                c.revealed = true;
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, texture: &Texture, camera_position: Vec2i) {
        for i in 0..self.elems.len() {
            let x = i as u32 % CHUNK_SIZE;
            let y = i as u32 / CHUNK_SIZE;
            let p0 = Vec2::new((x * CELL_SIZE) as f32, (y * CELL_SIZE) as f32);
            let p0 = p0 - camera_position.vec2();
            let p1 = p0 + Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32);

            let uv_size = texture.get_sprite_size();
            
            let bckg = if !self.elems[i].revealed { SPRITE_HIDDEN } else { SPRITE_0 };
            let uv0 = texture.get_uv(bckg.0, bckg.1);
            renderer.push_2d_sprite(p0, p1, uv0, uv0 + uv_size); // Background
            if !self.elems[i].revealed { 
                if self.elems[i].flag {
                    let uv0 = texture.get_uv(SPRITE_FLAG.0, SPRITE_FLAG.1);
                    renderer.push_2d_sprite(p0, p1, uv0, uv0 + uv_size); // Background
                }
             } else {
                match self.elems[i].contents {
                    CellContents::Empty(nb) => {
                        if nb != 0 {
                            let uv0 = texture.get_uv((nb - 1) % 4, (nb - 1) /4);
                            renderer.push_2d_sprite(p0, p1, uv0, uv0 + uv_size); 
                        }
                    },
                    CellContents::Mine => {
                        let uv0 = texture.get_uv(SPRITE_MINE.0, SPRITE_MINE.1);
                        renderer.push_2d_sprite(p0, p1, uv0, uv0 + uv_size);
                    }
                };
            }
        }
    }

    pub fn get_cell(&self, pos: (u32, u32)) -> Option<&Cell> {
        if pos.0 >= CHUNK_SIZE || pos.1 >= CHUNK_SIZE {
            return None;
        }
        self.elems.get(Chunk::idx(pos.0, pos.1))
    }

    pub fn get_cell_mut(&mut self, pos: (u32, u32)) -> Option<&mut Cell> {
        self.elems.get_mut(Chunk::idx(pos.0, pos.1))
    }
}

struct Grid {
    chunk: Chunk
}

impl Grid {
    pub fn new() -> Grid {
         Grid {
             chunk : Chunk::new(Vec2i::new(0, 0), DENSITY)
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, texture: &Texture, camera_position: Vec2i) {
        texture.bind();
        self.chunk.draw(renderer, texture, camera_position);
    }

    fn get_cell(&self, pos: Vec2i) -> Option<&Cell> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        self.chunk.get_cell((pos.x as u32, pos.y as u32))
    }

    fn get_cell_mut(&mut self, pos: Vec2i) -> Option<&mut Cell> {
        self.chunk.get_cell_mut((pos.x as u32, pos.y as u32))
    }

    pub fn reveal(&mut self, world_pos: Vec2i) -> bool {
        let pos = world_pos / CELL_SIZE as i32;
        let mut cell = self.get_cell_mut(pos).unwrap();
        cell.revealed = true;
        if let CellContents::Mine = cell.contents { return true; }
        if let CellContents::Empty(x) = cell.contents { if x != 0 { return false; } }
        self.reveal_recurse(pos);
        return false;
    }

    pub fn reveal_recurse(&mut self, pos: Vec2i) {
        let mut cell = self.get_cell_mut(pos).unwrap();
        cell.revealed = true;

        fn check_cell(cell: &Cell) -> bool {
            if cell.revealed == true { 
                return false;
            } 
            match cell.contents {
                CellContents::Empty(_) => true,
                CellContents::Mine => false,
            }
        }

        let adj = [Vec2i::new(-1, 0), Vec2i::new(1, 0), Vec2i::new(0, -1), Vec2i::new(0, 1)];
        for p in adj {
            let pos = pos + p;
            if let Some(c) = self.get_cell(pos) {
                if check_cell(c) {
                    self.reveal_recurse(pos);
                }
            }  
        }
    }

    pub fn flag(&mut self, world_pos: Vec2i) {
        if let Some(cell) = self.get_cell_mut(world_pos) {
            cell.flag = !cell.flag;
        }
    }

    pub fn show_all_mines(&mut self) {
        self.chunk.show_all_mines();
    }
}
