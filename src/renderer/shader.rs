use gl;
use gl::types::*;

use std::ffi::{CStr, CString};

use crate::math::Mat4;

use super::Error;

fn build_shader_part(source: &CStr, kind: GLenum) -> Result<GLuint, Error> {
    let shader = unsafe { gl::CreateShader(kind) };
    if shader == 0 {
        return Err(Error::CreateError);
    }
    unsafe {
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }
    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        unsafe {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let buffer: Vec<u8> = vec![0; len as usize + 1];
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_ptr() as *mut GLchar,
            );
            return Err(Error::ShaderCompileError(
                CString::from_vec_unchecked(buffer)
                    .to_string_lossy()
                    .into_owned(),
                kind,
            ));
        }
    }
    Ok(shader)
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(vtx_source: &str, frag_source: &str) -> Result<Shader, Error> {
        let vtx_source = &CString::new(vtx_source)?;
        let frag_source = &CString::new(frag_source)?;

        let vtx = build_shader_part(vtx_source, gl::VERTEX_SHADER)?;
        let frag = build_shader_part(frag_source, gl::FRAGMENT_SHADER)?;

        let program = unsafe { gl::CreateProgram() };
        if program == 0 {
            return Err(Error::CreateError);
        }

        unsafe {
            gl::AttachShader(program, vtx);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);
            let mut success: GLint = 1;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let buffer: Vec<u8> = vec![0; len as usize + 1];
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_ptr() as *mut GLchar,
                );
                return Err(Error::ProgramLinkError(
                    CString::from_vec_unchecked(buffer)
                        .to_string_lossy()
                        .into_owned(),
                ));
            }
        }

        unsafe {
            gl::DeleteShader(vtx);
            gl::DeleteShader(frag);
        }

        Ok(Shader { id: program })
    }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<GLint, Error> {
        self.set_used();
        let name = &CString::new(name)?;
        let loc;
        unsafe {
            loc = gl::GetUniformLocation(self.id, name.as_ptr());
        }
        if loc != -1 {
            Ok(loc)
        } else {
            Err(Error::UniformNotFound)
        }
    }

    pub fn set_uniform<T>(&self, name: &str, value: T) -> Result<(), Error>
    where
        T: UniformType,
    {
        self.set_used();
        let loc = self.get_uniform_location(name)?;
        T::set_uniform(value, loc)
    }
}

pub trait UniformType {
    fn set_uniform(val: Self, loc: GLint) -> Result<(), Error>;
}

impl UniformType for Mat4 {
    fn set_uniform(val: Self, loc: GLint) -> Result<(), Error> {
        let mat: [[f32; 4]; 4] = Into::<[[f32; 4]; 4]>::into(val);

        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_ptr() as *const f32);
        }
        Ok(())
    }
}

impl UniformType for &Mat4 {
    fn set_uniform(val: Self, loc: GLint) -> Result<(), Error> {
        let mat: [[f32; 4]; 4] = Into::<[[f32; 4]; 4]>::into(*val);

        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_ptr() as *const f32);
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
