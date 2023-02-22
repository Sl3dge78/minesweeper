use std::f32::consts::PI;

use sdl2::keyboard::Scancode;

use crate::{game::*, math::*, room::Room};

pub struct Player {
    position: Vec3,
    eye_height: f32,
    velocity: Vec3,
    yaw: f32,
    pitch: f32,
    forward: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        let yaw = -PI / 2.0;
        let pitch = PI / 2.0;
        let forward = forward_from_yaw_pitch(yaw, pitch);
        Player {
            position: Vec3::new(0.0, 0.0, 0.0),
            eye_height: 1.7,
            velocity: Vec3::zero(),
            forward,
            yaw,
            pitch,
        }
    }
}

impl Player {
    pub fn update(game_state: &mut GameState, input: &Input) {
        let player = &mut game_state.player;
        player.handle_movement(&game_state.room, game_state.delta_time, input);

        if input.mouse.middle() {
            game_state.enemies.push(Enemy::new());
        }

        if input.mouse.left() {
            let shoot_height = 1.2;
            game_state.bullets.push(Bullet::new(
                player.position + UP * shoot_height + player.forward * 0.2,
                player.forward,
            ));
        }
    }

    fn handle_movement(&mut self, room: &Room, delta_time: f32, input: &Input) {
        // View
        self.yaw += input.mouse.x() as f32 * 0.2 * delta_time;
        self.yaw %= 2.0 * PI;
        self.pitch += input.mouse.y() as f32 * 0.2 * delta_time;
        self.pitch = self.pitch.clamp(f32::EPSILON, PI - f32::EPSILON);

        self.forward = forward_from_yaw_pitch(self.yaw, self.pitch);
        let flat_fwd = self.forward.mul_element_wise(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        });
        let right = flat_fwd.cross(UP);

        // Horizontal Movement
        let max_speed = 10.0;
        let is_grounded = self.is_grounded();

        let mut hor_vel = self.velocity;
        hor_vel.y = 0.0;
        let mut acceleration = Vec3::zero();

        if input.keyboard.is_scancode_pressed(Scancode::W) {
            acceleration += flat_fwd;
        }
        if input.keyboard.is_scancode_pressed(Scancode::S) {
            acceleration -= flat_fwd;
        }
        if input.keyboard.is_scancode_pressed(Scancode::D) {
            acceleration += right;
        }
        if input.keyboard.is_scancode_pressed(Scancode::A) {
            acceleration -= right;
        }

        let acceleration_speed = if is_grounded { 200.0 } else { 20.0 };

        if acceleration.magnitude() > 0.0 {
            hor_vel += acceleration.normalize() * acceleration_speed * delta_time;
            if hor_vel.magnitude() > max_speed {
                hor_vel = hor_vel.normalize() * max_speed;
            }
        }
        self.velocity.x = hor_vel.x;
        self.velocity.z = hor_vel.z;

        // Vertical movement
        let jump_vel = 5.0;
        if is_grounded {
            if input.keyboard.is_scancode_pressed(Scancode::Space) {
                self.velocity.y = jump_vel;
            }
        }
        self.velocity.y -= 9.81 * delta_time;

        // Collision
        if self.velocity.magnitude() > 0.0 {
            let new_pos = self.position + self.velocity * delta_time;

            // Check collision
            if room.is_in(self.position + (self.velocity.mul_element_wise(RIGHT) * delta_time)) {
                // X
                self.position.x = new_pos.x;
            }
            if room.is_in(self.position + (self.velocity.mul_element_wise(FORWARD) * delta_time)) {
                // Z
                self.position.z = new_pos.z;
            }
            if new_pos.y >= 0.0 {
                self.position.y = new_pos.y;
            } else {
                self.position.y = 0.0;
            }
        }

        // Drag
        let drag = if self.is_grounded() { 10.0 } else { 1.1 };

        let mut drag_dir = self.velocity;
        drag_dir.y = 0.0;
        let amount = max_speed * drag * delta_time;
        if drag_dir.magnitude() > amount {
            self.velocity -= drag_dir.normalize() * amount;
        } else {
            self.velocity.x = 0.0;
            self.velocity.z = 0.0;
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.position.y <= 0.0
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            (self.position + UP * self.eye_height).point3(),
            (self.position + UP * self.eye_height + self.forward).point3(),
            UP,
        )
    }
}
