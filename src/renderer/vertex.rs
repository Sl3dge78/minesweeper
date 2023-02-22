
use crate::math::{Vec3, Vec2};
use cgmath::{Vector3, Zero};

use gl::types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vector3::zero(),
            normal: Vector3::zero(),
            uv: Vec2::zero(),
            color: Vec3::zero(),
        }
    }
}

impl Vertex {
    pub fn enable_attrib() {
        unsafe {
            let mut offset = 0;
            let dumb_vertex: Vertex = Default::default();

            gl::EnableVertexAttribArray(0); // location = 0 - POSITION
            gl::VertexAttribPointer(0, 3,
                gl::FLOAT, gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                offset as *const gl::types::GLvoid,
            );

            offset += std::mem::size_of_val(&dumb_vertex.pos);

            gl::EnableVertexAttribArray(1); // location = 1 - NORMAL
            gl::VertexAttribPointer(1, 3,
                 gl::FLOAT, gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                offset as *const gl::types::GLvoid,
            );
            offset += std::mem::size_of_val(&dumb_vertex.normal);

            gl::EnableVertexAttribArray(2); // location = 2 - UV
            gl::VertexAttribPointer(2, 2,
                gl::FLOAT, gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                offset as *const gl::types::GLvoid,
            );
            offset += std::mem::size_of_val(&dumb_vertex.uv);

            gl::EnableVertexAttribArray(3); // location = 3 - Color
            gl::VertexAttribPointer(3, 3,
                gl::FLOAT, gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                offset as *const gl::types::GLvoid,
            );
        }
    }
}