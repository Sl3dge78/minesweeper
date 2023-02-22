use super::*;

#[allow(dead_code)]
pub struct RenderTexture {
    framebuffer: GLuint,
    color_texture: Texture,
    depth_stencil_texture: Texture,
    width: i32,
    height: i32,
}

#[allow(dead_code)]
impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Result<RenderTexture, Error> {
        let mut framebuffer: GLuint = 0;
        let color_texture = Texture::new();
        let depth_stencil_texture = Texture::new();
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            
            // Color
            color_texture.bind();
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_texture.handle(), 0);

            depth_stencil_texture.bind();
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, depth_stencil_texture.handle(), 0);
        }

        Ok(RenderTexture { framebuffer, color_texture, depth_stencil_texture, width, height })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.framebuffer);
        }
    }
}
