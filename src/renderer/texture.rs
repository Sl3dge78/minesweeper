use gl::types::*;
use std::fs::File;
use png::{Decoder, Reader};
use crate::resources::{ResourceLoader, ResourceKind};

pub struct Texture {
    handle: GLuint,
    pub width: u32,
    pub height: u32,
}

pub enum Filter {
    Linear,
    Nearest
}

impl Filter {
    pub fn get_gl_val(&self) -> i32 {
        match self {
            Filter::Linear => gl::LINEAR as i32,
            Filter::Nearest => gl::NEAREST as i32,
        }
    }
}

#[allow(dead_code)]
impl Texture {

    pub fn new() -> Texture {
        let mut handle: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut handle);
        }
        Self { handle, width: 0, height: 0}
    }

    pub fn from_image(path: &str) -> Option<Texture> {
        let mut tex = Texture::new();
        let file = File::open(path).ok()?;
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().ok()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).ok()?;
        
        let format = match info.color_type {
            png::ColorType::Grayscale => gl::R8,
            png::ColorType::Rgb => gl::RGB,
            png::ColorType::Rgba => gl::RGBA,
            _ => return None,
        };
        tex.set_data(info.width, info.height, format, &buf);
        // tex.bind();
        // unsafe {
        //     gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, info.width as i32, info.height as i32, 0, format, gl::UNSIGNED_BYTE, buf.as_ptr() as *const GLvoid);
        // }
        Texture::set_filter(Filter::Linear);
        tex.width = info.width;
        tex.height = info.height;
        Some(tex)
    }

    pub fn set_data(&mut self, w: u32, h: u32, format: GLenum, buf: &[u8]) {
        self.bind();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, format, gl::UNSIGNED_BYTE, buf.as_ptr() as *const GLvoid);
        }
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

    pub fn set_filter(filter: Filter) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter.get_gl_val()); 
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter.get_gl_val());
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

impl ResourceLoader for Texture {
    const EXT : &'static str = "png";
    fn load_resource(path: &str) -> Option<ResourceKind> {
        Some(ResourceKind::Texture(Texture::from_image(path)?))
    }
}
