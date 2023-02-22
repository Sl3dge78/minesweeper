use gl::types::GLuint;

pub struct Texture {
    handle: GLuint,
}

#[allow(dead_code)]
impl Texture {
    pub fn new() -> Texture {
        let mut handle: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut handle);
        }
        Self { handle}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn handle(&self) -> GLuint {
        self.handle
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}
