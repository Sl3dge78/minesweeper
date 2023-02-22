pub use cgmath::{prelude::*, Basis3, Matrix4, Point3 as P3, Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Point3 = P3<f32>;

pub static UP: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub static DOWN: Vec3 = Vec3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};
pub static FORWARD: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
pub static BACK: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
pub static RIGHT: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub static LEFT: Vec3 = Vec3 {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};
pub static ONE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub fn forward_from_yaw_pitch(yaw: f32, pitch: f32) -> Vec3 {
    Vec3 {
        x: pitch.sin() * yaw.cos(),
        y: pitch.cos(),
        z: pitch.sin() * yaw.sin(),
    }
}

pub trait IntoVec3<T> {
    fn vec3(&self) -> T;
}

impl IntoVec3<Vec3> for Point3 {
    fn vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

pub trait IntoPoint3<T> {
    fn point3(&self) -> T;
}

impl IntoPoint3<Point3> for Vec3 {
    fn point3(&self) -> Point3 {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
