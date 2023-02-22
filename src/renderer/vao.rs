use super::*;
pub struct VertexArray {
    handle: GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VertexArray { handle: vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
        }
    }
}