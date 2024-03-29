#![allow(unused_imports)]

use gl::types::*;
use sdl2::{video::*, sys::{SDL_GL_SetAttribute, SDL_GLattr, SDL_GLprofile}};
use crate::math::*;

mod shader;
use shader::*;

mod renderer;
pub use renderer::*;

mod mesh;
use mesh::*;

mod buffer;
use buffer::*;

mod fbo;
use fbo::*;

mod vertex;
use vertex::*;

mod vao;
use vao::*;

mod error;
use error::Error::{self, *};

mod texture;
pub use texture::*;
