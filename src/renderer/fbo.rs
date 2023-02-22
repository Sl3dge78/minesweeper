use super::*;

#[allow(dead_code)]
pub struct RenderTexture {
    framebuffer: GLuint,
    color_texture: GLuint,
    depth_stencil_texture: GLuint,
    width: i32,
    height: i32,
}

impl RenderTexture {
    pub fn new(width: i32, height: i32) -> Result<RenderTexture, Error> {
        let mut framebuffer: GLuint = 0;
        let mut color_texture: GLuint = 0;
        let mut depth_stencil_texture: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            
            // Color
            gl::GenTextures(1, &mut color_texture);
            gl::BindTexture(gl::TEXTURE_2D, color_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_texture, 0);

            gl::GenTextures(1, &mut depth_stencil_texture);
            gl::BindTexture(gl::TEXTURE_2D, depth_stencil_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, depth_stencil_texture, 0);
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
            gl::DeleteTextures(1, &self.color_texture);
            gl::DeleteTextures(1, &self.depth_stencil_texture);
        }
    }
}
