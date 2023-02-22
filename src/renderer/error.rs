use super::*;

use std::fmt::Display;

pub enum Error {
    NulError,
    UnableToCreateContext,
    UniformNotFound,
    CreateError,
    ShaderCompileError(String, GLenum),
    ProgramLinkError(String),
    FrameBufferError,
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Self {
        Error::NulError
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NulError => write!(f, "NulError"),
            Self::UniformNotFound => write!(f, "UniformNotFound"),
            Self::CreateError => write!(f, "CreateError"),
            Self::ShaderCompileError(_, _) => write!(f, "ShaderCompileError: {}", self),
            Self::ProgramLinkError(_) => write!(f, "ProgramLinkError: {}", self),
            Self::UnableToCreateContext => write!(f, "CreateError"),
            Self::FrameBufferError => write!(f, "Error while creating framebuffer")
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NulError => write!(f, "Unable to convert to C String"),
            Error::UniformNotFound => write!(f, "Uniform not found"),
            Error::CreateError => write!(f, "Unable to create resource"),
            Error::ShaderCompileError(msg, kind) => {
                write!(
                    f,
                    "Unable to compile {}: {}",
                    match kind {
                        &gl::VERTEX_SHADER => "Vertex shader",
                        &gl::FRAGMENT_SHADER => "Fragment shader",
                        _ => "Unknown",
                    },
                    msg
                )
            }
            Error::ProgramLinkError(msg) => write!(f, "Link error: {}", msg),
            Error::UnableToCreateContext => write!(f, "CreateError"),
            Error::FrameBufferError => write!(f, "Error while creating framebuffer")
        }
    }
}
