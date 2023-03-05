
use sdl2::{event::Event, keyboard::Scancode};

use crate::{math::*, renderer::*, input::*, sprite_sheet::SpriteSheet};

enum State {
    Playing,
    Lost,
    Won,
}

pub struct GameState {
    pub delta_time: f32,
    grid: Grid,
    sprites: Texture,
    state: State,
}

const SPRITE_HIDDEN: (i32, i32) = (0, 2);
const SPRITE_FLAG: (i32, i32) = (1, 2);
const SPRITE_MINE: (i32, i32) = (2, 2);
const SPRITE_0: (i32, i32) = (3, 2);

const NB_MINES: u32 = 20;

impl Default for GameState {
    fn default() -> Self {
        GameState {
            delta_time : 0.0,
            grid: Grid::generate(16, NB_MINES),
            sprites: Texture::from_image("res/sprites.png").expect("Unable to load sprites."),
            state: State::Playing,
        }
    }
}

impl GameState {
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
                            self.grid = Grid::generate(16, NB_MINES);
                            self.state = State::Playing;
                        },
                        _ => {},
                    }   
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
        self.sprites.bind();
        self.grid.draw(renderer, &self.sprites);
    }

    pub fn on_left_click(&mut self, x: i32, y: i32) {
        let (x,y) = self.grid.get_coords(x, y);
        if self.grid.reveal((x, y)) {
            self.loose();
        } else {
            self.check_win();
        }
    }

    pub fn on_right_click(&mut self, x: i32, y: i32) {
        let (x,y) = self.grid.get_coords(x, y);
        self.grid.flag(x, y);
    }

    fn check_win(&mut self) {
        for c in &self.grid.elems {
            match c.contents {
                CellContents::Empty(_) => if !c.revealed { return; },
                CellContents::Mine => {},
            }
        }
        self.state = State::Won;
        println!("You won!");
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

struct Grid {
    elems : Vec<Cell>,
    cell_offset: Vec2,
    size: u32,
}

impl Grid {
    pub fn generate(size: u32, nb_mines: u32) -> Grid {
        let mut result = Grid {
            elems: Vec::new(),
            cell_offset: Vector2::new(0.0, 0.0),
            size
        };
        result.elems.resize((size * size) as usize, Default::default());

        for _ in 0..nb_mines {
            loop {
                let x = rand::random::<u32>() % size;
                let y = rand::random::<u32>() % size;
                let index: usize = (x + y * size) as usize;
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

    fn get_cell(&self, pos: (u32, u32)) -> &Cell {
        let idx = Grid::get_index(self.size, pos.0, pos.1);
        return &self.elems[idx];
    }

    fn get_cell_mut(&mut self, pos:(u32, u32)) -> &mut Cell {
        let idx = Grid::get_index(self.size, pos.0, pos.1);
        return &mut self.elems[idx];
    }

    fn get_index(size: u32, x: u32, y: u32) -> usize {
        (x + y * size) as usize
    }

    fn place_mine(&mut self, x: u32, y: u32) {
        let idx = Grid::get_index(self.size, x, y);
        self.elems[idx].contents = CellContents::Mine;
        let start_x = if x == 0 { 0 } else { x - 1 };
        let end_x = if x == self.size - 1 { self.size - 1 } else { x + 1 };

        let start_y = if y == 0 { 0 } else { y - 1};
        let end_y = if y == self.size - 1 { self.size - 1 } else { y + 1};
        for x2 in start_x..=end_x {
            for y2 in start_y..=end_y {
                if x2 == x && y2 == y { continue; }
                let idx = Grid::get_index(self.size, x2, y2);
                if let CellContents::Empty(ref mut nb) = self.elems[idx].contents {
                    *nb += 1;
                }
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, texture: &Texture) {
        for i in 0..self.elems.len() {
            let x = i as u32 % self.size;
            let y = i as u32 / self.size;
            let p0 = Vec2::new(self.cell_offset.x + (x * CELL_SIZE) as f32, self.cell_offset.y + (y * CELL_SIZE) as f32);
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

    pub fn get_coords(&self, x: i32, y: i32) -> (u32, u32) {
        let x = ((x as f32 - self.cell_offset.x) / (CELL_SIZE as f32)) as u32; 
        let y = ((y as f32 - self.cell_offset.y) / (CELL_SIZE as f32)) as u32;
        return (x, y);
    }

    pub fn reveal(&mut self, pos: (u32, u32)) -> bool {
        // The one we clicked on
        let mut cell = self.get_cell_mut(pos);
        cell.revealed = true;
        if let CellContents::Mine = cell.contents { return true; }
        if let CellContents::Empty(x) = cell.contents { if x != 0 { return false; } }

        fn check_cell(cell: &Cell) -> bool {
            if cell.revealed == true { 
                return false;
            } 
            match cell.contents {
                CellContents::Empty(_) => true,
                CellContents::Mine => false,
            }
        }

        if pos.0 != 0 {
            let pos = (pos.0-1, pos.1);
            if check_cell(self.get_cell(pos)) {
                self.reveal(pos);
            }
        }
        if pos.0 + 1 != self.size {
            let pos = (pos.0+1, pos.1);
            if check_cell(self.get_cell(pos)) {
                self.reveal(pos);
            }
        }
        if pos.1 != 0 {
            let pos = (pos.0, pos.1-1);
            if check_cell(self.get_cell(pos)) {
                self.reveal(pos);
            }
        }
        if pos.1 + 1 != self.size {
            let pos = (pos.0, pos.1+1);
            if check_cell(self.get_cell(pos)) {
                self.reveal(pos);
            }
        }
        
        return false;
    }

    pub fn flag(&mut self, x: u32, y: u32) {
        let idx = Grid::get_index(self.size, x, y);
        self.elems[idx].flag = !self.elems[idx].flag;
    }

    pub fn show_all_mines(&mut self) {
        for mut c in &mut self.elems {
            if let CellContents::Mine = c.contents {
                c.revealed = true;
            }
        }
    }

/*

        for (x, y) in Grid::iter_neighbors(self.size, x, y) {
            let id = Grid::get_index(self.size, x, y);
            if let CellContents::Empty(nb) = self.elems[id].contents {
                 if nb != 0 { continue; }
            } else {
                continue; 
            }
            self.reveal(x,y);
        }
    pub fn iter_neighbors(size: u32, x: u32, y: u32) -> Vec<(u32, u32)> {
        let a = if x == 0 {
            None
        } else {
            Some((x-1, y))
        };
        let b = if y == 0 {
            None
        } else {
            Some((x, y-1))
        };
        let c = if x + 1 == size {
            None
        } else {
            Some((x+1, y))
        };
        let d = if y + y == size {
            None
        } else {
            Some((x, y+1))
        };
        a.into_iter().chain(b).chain(c).chain(d)
    }
    */
}
