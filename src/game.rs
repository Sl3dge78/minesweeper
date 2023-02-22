use sdl2::{keyboard::KeyboardState, mouse::RelativeMouseState, EventPump};

use crate::{math::*, player::Player, renderer::*, room::Room};

pub struct GameState {
    pub delta_time: f32,
    pub player: Player,
    pub room: Room,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            room: Room::new(20.0, 5.0),
            delta_time: Default::default(),
            player: Default::default(),
            enemies: Vec::new(),
            bullets: Vec::new(),
        }
    }
}

pub struct Input<'a> {
    pub keyboard: KeyboardState<'a>,
    pub mouse: RelativeMouseState,
}

impl<'a> Input<'a> {
    pub fn from_pump(event_pump: &'a EventPump) -> Input<'a> {
        Input {
            keyboard: event_pump.keyboard_state(),
            mouse: event_pump.relative_mouse_state(),
        }
    }
}

impl GameState {
    pub fn update(&mut self, input: &Input) {

        Player::update(self, input);
        for bullet in self.bullets.iter_mut() {
            bullet.update(self.delta_time);
        }
        for e in self.enemies.iter_mut() {
            e.update(&mut self.bullets);
        }
        self.enemies.retain(|e| e.health > 0);
        self.bullets.retain(|b| b.delete == false);
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_draw();
        renderer.view = self.player.view();
        self.room.draw(renderer);
        for e in self.enemies.iter() {
            e.draw(renderer);
        }
        for b in self.bullets.iter() {
            b.draw(renderer);
        }
    }
}

pub struct Enemy {
    position: Vec3,
    health: i32,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            position: Vec3::new(0.0, 1.0, 0.0),
            health: 4,
        }
    }

    pub fn update(&mut self, bullets: &mut Vec<Bullet>) {
        for b in bullets.iter_mut() {
            if (b.position - self.position).magnitude() < 0.5 {
                self.health -= 1;
                b.delete = true;
                return;
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.set_model_matrix(&Mat4::from_translation(self.position));
        renderer.push_cube(Vec3::new(1.0, 0.0, 0.0));
    }
}
pub struct Bullet {
    position: Vec3,
    direction: Vec3,
    delete: bool,
}

impl Bullet {
    pub fn new(position: Vec3, direction: Vec3) -> Bullet {
        Bullet {
            position,
            direction,
            delete: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let velocity = 20.0;
        self.position += self.direction * velocity * delta_time;
        if self.position.magnitude() > 20.0 {
            self.delete = true;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mat = Mat4::from_translation(self.position) * Mat4::from_scale(0.1);
        renderer.set_model_matrix(&mat);
        renderer.push_cube(Vec3::new(0.0, 1.0, 0.0));
    }
}

/*
use bitflags::bitflags;
bitflags! {
    #[derive(Default)]
    struct EntityFlags: u32 {
        const should_delete = 0b1;
    }
}

#[derive(Default)]
pub struct EntityData {
    flags: EntityFlags
}

pub enum Entity {
    Bullet(Bullet),
    Enemy(Enemy),
}

*/
