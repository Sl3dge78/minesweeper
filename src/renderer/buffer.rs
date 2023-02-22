use super::*;

pub struct Buffer {
    handle: GLuint,
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut buf: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut buf);
        }
        Buffer { handle: buf }
    }

    pub fn set_data(&self, data: &[f32]) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STREAM_DRAW,
            );
        }
    }

    pub fn set_data_from_vertices(&self, data: &[Vertex]) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        }
    }
}