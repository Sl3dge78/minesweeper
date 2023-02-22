use std::f32::consts::PI;

use crate::{math::*, renderer::Renderer};

#[derive(Debug)]
pub struct Room {
    radius: f32,
    height: f32,
}

#[allow(dead_code)]
impl Room {
    pub fn new(radius: f32, height: f32) -> Room {
        Room { radius, height }
    }

    pub fn is_in(&self, point: Vec3) -> bool {
        let distance = Vec3::distance(Vec3::zero(), point);
        distance < self.radius - 0.2 // safety margin
    }

    fn push_door(renderer: &mut Renderer, axis: Vec3, x: f32, y: f32, z: f32) {
        renderer.push_aligned_quad(
            axis,
            Vec2 {
                x: -x / 2.0,
                y: -y / 2.0,
            },
            Vec2 {
                x: -x / 4.0,
                y: y / 2.0,
            },
            z / 2.0,
        );
        renderer.push_aligned_quad(
            axis,
            Vec2 {
                x: -x / 4.0,
                y: 0.0,
            },
            Vec2 {
                x: x / 4.0,
                y: y / 2.0,
            },
            z / 2.0,
        );
        renderer.push_aligned_quad(
            axis,
            Vec2 {
                x: x / 4.0,
                y: -y / 2.0,
            },
            Vec2 {
                x: x / 2.0,
                y: y / 2.0,
            },
            z / 2.0,
        );
    }

    fn push_wall(renderer: &mut Renderer, axis: Vec3, x: f32, y: f32, z: f32) {
        renderer.push_aligned_quad(
            axis,
            Vec2 {
                x: -x / 2.0,
                y: -y / 2.0,
            },
            Vec2 {
                x: x / 2.0,
                y: y / 2.0,
            },
            z / 2.0,
        );
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.set_model_matrix(&Mat4::identity());

        let nb_iterations = 20;
        let iter_size = 2.0 * PI / (nb_iterations as f32);

        for i in 0..nb_iterations {
            let ang_pos = iter_size * i as f32;
            let x1 = f32::cos(ang_pos) * self.radius;
            let z1 = f32::sin(ang_pos) * self.radius;
            let x2 = f32::cos(ang_pos + iter_size) * self.radius;
            let z2 = f32::sin(ang_pos + iter_size) * self.radius;

            let p0 = Point3 {
                x: x1,
                y: 0.0,
                z: z1,
            };
            let p3 = Point3 {
                x: x2,
                y: 0.0,
                z: z2,
            };

            renderer.push_quad_corners(
                p0,
                Point3 { x: x1, y: self.height, z: z1, },
                Point3 { x: x2, y: self.height, z: z2, },
                p3,
                Vec3::cross((p3 - p0).normalize(), UP),
                Vec3::new(1.0, 1.0, 1.0),
            );
        }

        // Draw floor & ceiling as a simple quad
        renderer.push_aligned_quad(
            UP,
            Vec2 { x: -self.radius, y: -self.radius, },
            Vec2 { x: self.radius, y: self.radius, },
            self.height,
        );
        renderer.push_aligned_quad(
            UP,
            Vec2 { x: -self.radius, y: -self.radius, },
            Vec2 { x: self.radius, y: self.radius, },
            0.0,
        );
    }

    /*
    pub fn draw(&self, renderer: &mut Renderer) {
        let mat = Mat4::from_translation(self.center + (UP * (self.height / 2.0)));
        renderer.set_model_matrix(&mat);
        // Floor
        Self::push_wall(renderer, DOWN, self.width, self.length, self.height);
        Self::push_wall(renderer, UP, self.width, self.length, self.height);
        if let Some(_w) = self.west_exit {
            Self::push_door(renderer, RIGHT, self.length, self.height, self.width);
        } else {
            Self::push_wall(renderer, RIGHT, self.length, self.height, self.width);
        }
        if let Some(_w) = self.east_exit {
            Self::push_door(renderer, LEFT, self.length, self.height,self.width);
        } else {
            Self::push_wall(renderer, LEFT, self.length, self.height, self.width);
        }
        if let Some(_w) = self.north_exit {
            Self::push_door(renderer, FORWARD, self.width, self.height, self.length);
        } else {
            Self::push_wall(renderer, FORWARD, self.width, self.height, self.length);
        }
        if let Some(_w) = self.south_exit {
            Self::push_door(renderer, BACK, self.width, self.height, self.length);
        } else {
            Self::push_wall(renderer, BACK, self.width, self.height, self.length);
        }
    }
    */
}
