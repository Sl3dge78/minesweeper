
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
    camera: Camera,
}

struct Camera {
    position: Vec2i,
    zoom: i32,
}

impl Camera {
    pub fn cell_size (&self) -> f32 {
        self.zoom as f32
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2i {
        return ((self.position.vec2() + pos) / self.cell_size()).vec2i();
    }
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
            camera: Camera { position: Vec2i::new(0, 0), zoom: CELL_SIZE },
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
            Event::MouseWheel { y, .. } => {
                if let State::Playing = self.state {
                    self.camera.zoom += y;
                    if self.camera.zoom < 4 { self.camera.zoom = 4; }
                    if self.camera.zoom > 32 { self.camera.zoom = 32; }
                }
            }
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
            self.camera.position += Vec2i::new(- input.rel_mouse.x(), - input.rel_mouse.y());
            self.grid.update_chunks(&self.camera);
        }

    }

    pub fn draw(&self, renderer: &mut Renderer, resources: &Resources) {
        Renderer::clear(Vec4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0});
        renderer.begin_2d();
        renderer.default_texture();
        
        self.grid.draw(renderer, resources.get("./res/sprites.png").as_texture(), &self.camera);
    }


    pub fn on_left_click(&mut self, x: i32, y: i32) {
        let pos = self.camera.screen_to_world(Vec2::new(x as f32, y as f32));
        if self.grid.reveal(pos) {
            self.loose();
        }
    }

    pub fn on_right_click(&mut self, x: i32, y: i32) {
        let pos = self.camera.screen_to_world(Vec2::new(x as f32, y as f32));
        self.grid.flag(pos);
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

const CELL_SIZE: i32 = 16;
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
        println!("Generating chunk {:?}", position);
        let nb_mines: u32 = (density * (CHUNK_LEN) as f32) as u32;
        for _ in 0..nb_mines {
            loop {
                let x = rand::random::<u32>() % CHUNK_SIZE as u32;
                let y = rand::random::<u32>() % CHUNK_SIZE as u32;
                let idx= Chunk::idx(x, y);
                match result.elems[idx].contents {
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

    pub fn draw(&self, renderer: &mut Renderer, texture: &Texture, camera: &Camera) {
        let cell_size = camera.cell_size();
        let origin = self.position.vec2() * CHUNK_SIZE as f32 * cell_size;
        let origin = origin - camera.position.vec2();
        for i in 0..self.elems.len() {
            let x = i as u32 % CHUNK_SIZE;
            let y = i as u32 / CHUNK_SIZE;
            let p0 = Vec2::new(x as f32 * cell_size, y as f32 * cell_size) + origin;
            let p1 = p0 + Vec2::new(cell_size, cell_size);

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
    chunks: Vec<Chunk>
}

impl Grid {
    pub fn new() -> Grid {
         Grid {
             chunks: Vec::new()
        }
    }

    pub fn update_chunks(&mut self, camera: &Camera) {
        // Calculate the cameras extent in chunks. 
        // We approximate with the following values for the default zoom level:
        const NB_CHUNKS_HEIGHT : f32 = 4.0;
        const NB_CHUNKS_WIDTH : f32 = 5.0;

        let nb_h = NB_CHUNKS_HEIGHT * camera.cell_size() / CELL_SIZE as f32;
        let nb_w = NB_CHUNKS_WIDTH  * camera.cell_size() / CELL_SIZE as f32;
        let min_extent = camera.position / (camera.cell_size() as i32 * CHUNK_SIZE as i32);
        let min_extent = min_extent - Vec2i::new(1, 1);
        let max_extent = min_extent + Vec2i::new(nb_h.ceil() as i32, nb_w.ceil() as i32);
        let max_extent = max_extent + Vec2i::new(2, 0);
        self.chunks.retain(|x| x.position.x >= min_extent.x && x.position.x <= max_extent.x && x.position.y >= min_extent.y && x.position.y <= max_extent.y);
        for x in min_extent.x..=max_extent.x {
            for y in min_extent.y..=max_extent.y {
                let pos = Vec2i::new(x, y);
                if let Some(_) = self.find_chunk(pos) {
                    continue;
                }
                self.chunks.push(Chunk::new(pos, DENSITY));
            }
        }

    }

    pub fn draw(&self, renderer: &mut Renderer, texture: &Texture, camera: &Camera) {
        texture.bind();
        for c in &self.chunks {
            c.draw(renderer, texture, camera);
        }
    }

    fn find_chunk(&self, chunk_coord: Vec2i) -> Option<&Chunk> {
        for c in &self.chunks {
            if c.position == chunk_coord {
                return Some(c);
            }
        }
        None
    }

    fn find_chunk_mut(&mut self, chunk_coord: Vec2i) -> Option<&mut Chunk> {
        for c in &mut self.chunks {
            if c.position == chunk_coord {
                return Some(c);
            }
        }
        None
    }

    fn get_cell(&self, pos: Vec2i) -> Option<&Cell> {
        let chunk = self.find_chunk(pos / CHUNK_SIZE as i32)?;
        let x = pos.x - chunk.position.x * CHUNK_SIZE as i32;
        let y = pos.y - chunk.position.y * CHUNK_SIZE as i32;
        chunk.get_cell((x as u32, y as u32))
    }

    fn get_cell_mut(&mut self, pos: Vec2i) -> Option<&mut Cell> {
        let chunk = self.find_chunk_mut(pos / CHUNK_SIZE as i32)?;
        let x = pos.x - chunk.position.x * CHUNK_SIZE as i32;
        let y = pos.y - chunk.position.y * CHUNK_SIZE as i32;
        chunk.get_cell_mut((x as u32, y as u32))
    }

    pub fn reveal(&mut self, pos: Vec2i) -> bool {
        let mut cell = self.get_cell_mut(pos).unwrap();
        cell.revealed = true;
        if let CellContents::Mine = cell.contents { return true; }
        if let CellContents::Empty(x) = cell.contents { if x != 0 { return false; } }
        self.reveal_recurse(pos, 0);
        return false;
    }

    pub fn reveal_recurse(&mut self, pos: Vec2i, depth: u32) {
        if depth >= 8 {
            return;
        }
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
                    self.reveal_recurse(pos, depth + 1);
                }
            }  
        }
    }

    pub fn flag(&mut self, pos: Vec2i) {
        if let Some(cell) = self.get_cell_mut(pos) {
            cell.flag = !cell.flag;
        }
    }

    pub fn show_all_mines(&mut self) {
        for c in &mut self.chunks {
            c.show_all_mines();
        }
    }
}
