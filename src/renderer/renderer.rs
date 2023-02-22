use super::*;

pub struct Renderer {
    _gl_context: GLContext,
    immediate_vertices: Vec<Vertex>,
    vbo: Buffer,
    vao: VertexArray,
    pub projection: Mat4,
    pub view: Mat4,
    shader: Shader,
    screen_space_shader: Shader,
    w: u32,
    h: u32,
    scale: u32,
}

impl Renderer {
    #[must_use]
    pub fn new(window: &Window, video: &sdl2::VideoSubsystem) -> Result<Renderer, Error> {
        unsafe {
            SDL_GL_SetAttribute(
                SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as i32,
            );
            SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 3);
        }
        let gl_context = if let Ok(x) = window.gl_create_context() {
            x
        } else {
            return Err(UnableToCreateContext);
        };
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let (w, h) = window.size();

        let scale = 5;

        Ok(Renderer {
            _gl_context: gl_context,
            immediate_vertices: Vec::<Vertex>::new(),
            vbo: Buffer::new(),
            vao: VertexArray::new(),
            projection: cgmath::perspective(
                cgmath::Deg(60.0),
                (w as f32) / (h as f32),
                0.1,
                1000.0,
            ),
            shader: Shader::from_source(
                include_str!("../triangle.vert"),
                include_str!("../triangle.frag"),
            )
            .unwrap(),
            view: Mat4::identity(),
            screen_space_shader: Shader::from_source(
                include_str!("../screen_space.vert"),
                include_str!("../screen_space.frag"),
            )
            .unwrap(),
            w,
            h,
            scale,
        })
    }

    #[inline]
    pub fn enable(what: GLenum) {
        unsafe {
            gl::Enable(what);
        }
    }

    #[inline]
    pub fn clear(color: Vec4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn update_window(&mut self, window: &Window) {
        (self.w, self.h) = window.size();
        self.projection = cgmath::perspective(
            cgmath::Deg(60.0),
            (self.w as f32) / (self.h as f32),
            0.1,
            1000.0,
        );
    }

    #[inline]
    pub fn push_vertex(&mut self, vertex: Vertex) {
        self.immediate_vertices.push(vertex);
    }

    pub fn push_quad_corners(
        &mut self,
        p0: Point3,
        p1: Point3,
        p2: Point3,
        p3: Point3,
        normal: Vec3,
        color: Vec3,
    ) {
        self.push_vertex(Vertex {
            pos: p0.vec3(),
            normal,
            uv: Vec2::new(0.0, 0.0),
            color,
        });
        self.push_vertex(Vertex {
            pos: p1.vec3(),
            normal,
            uv: Vec2::new(1.0, 0.0),
            color,
        });
        self.push_vertex(Vertex {
            pos: p2.vec3(),
            normal,
            uv: Vec2::new(1.0, 1.0),
            color,
        });
        self.push_vertex(Vertex {
            pos: p0.vec3(),
            normal,
            uv: Vec2::new(0.0, 0.0),
            color,
        });
        self.push_vertex(Vertex {
            pos: p2.vec3(),
            normal,
            uv: Vec2::new(1.0, 1.0),
            color,
        });
        self.push_vertex(Vertex {
            pos: p3.vec3(),
            normal,
            uv: Vec2::new(0.0, 1.0),
            color,
        });
    }

    pub fn push_cube(&mut self, color: Vec3) {
        self.push_quad_corners(
            Point3 {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            },
            Point3 {
                x: -0.5,
                y: -0.5,
                z: 0.5,
            },
            UP,
            color,
        ); // -Y
        self.push_quad_corners(
            Point3 {
                x: -0.5,
                y: 0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Point3 {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            DOWN,
            color,
        ); // +Y
        self.push_quad_corners(
            Point3 {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            Point3 {
                x: -0.5,
                y: 0.5,
                z: -0.5,
            },
            Point3 {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            Point3 {
                x: -0.5,
                y: -0.5,
                z: 0.5,
            },
            LEFT,
            color,
        ); // -X
        self.push_quad_corners(
            Point3 {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            Point3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Point3 {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            },
            RIGHT,
            color,
        ); // +X
        self.push_quad_corners(
            Point3 {
                z: -0.5,
                y: -0.5,
                x: -0.5,
            },
            Point3 {
                z: -0.5,
                y: 0.5,
                x: -0.5,
            },
            Point3 {
                z: -0.5,
                y: 0.5,
                x: 0.5,
            },
            Point3 {
                z: -0.5,
                y: -0.5,
                x: 0.5,
            },
            BACK,
            color,
        ); // -Z
        self.push_quad_corners(
            Point3 {
                z: 0.5,
                y: -0.5,
                x: -0.5,
            },
            Point3 {
                z: 0.5,
                y: 0.5,
                x: -0.5,
            },
            Point3 {
                z: 0.5,
                y: 0.5,
                x: 0.5,
            },
            Point3 {
                z: 0.5,
                y: -0.5,
                x: 0.5,
            },
            FORWARD,
            color,
        ); // +Z
    }

    pub fn push_aligned_quad(&mut self, axis: Vec3, p0: Vec2, p1: Vec2, z: f32) {
        let up = if axis == UP || axis == DOWN {
            FORWARD
        } else {
            UP
        };
        let right = if axis == RIGHT || axis == LEFT {
            FORWARD
        } else {
            RIGHT
        };

        let mx = right * p0.x;
        let px = right * p1.x;
        let my = up * p0.y;
        let py = up * p1.y;
        let z = axis * z;

        self.push_quad_corners(
            (mx + my + z).point3(),
            (px + my + z).point3(),
            (px + py + z).point3(),
            (mx + py + z).point3(),
            axis,
            Vec3::new(1.0, 1.0, 1.0),
        );
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        for v in &mesh.vertices {
            self.push_vertex(*v);
        }
    }

    pub fn flush(&mut self) {
        if self.immediate_vertices.len() == 0 {
            return;
        } // Nothing to draw

        self.vao.bind();
        self.vbo.bind();

        self.vbo.set_data_from_vertices(&self.immediate_vertices);
        Vertex::enable_attrib();
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                self.immediate_vertices.len().try_into().unwrap(),
            );
        }
        self.immediate_vertices.clear();
    }

    pub fn begin_draw(&mut self) {
        Renderer::clear(cgmath::Vector4 {
            x: 0.1,
            y: 0.0,
            z: 0.1,
            w: 1.0,
        });
        self.fbo.bind();
        self.shader.set_used();
        self.shader
            .set_uniform("Projection", self.projection)
            .unwrap();
        self.shader.set_uniform("View", self.view).unwrap();

        Renderer::enable(gl::DEPTH_TEST);
        unsafe {
            gl::Viewport(
                0,
                0,
                (self.w / self.scale) as i32,
                (self.h / self.scale) as i32,
            );
        }
    }

    pub fn end_draw(&mut self) {
        self.flush();
        FrameBuffer::unbind();
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, self.w as i32, self.h as i32);
        }
        self.screen_space_shader.set_used();
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.fbo.texture) }
        self.push_quad_corners(
            Point3::new(-1.0, -1.0, 0.0),
            Point3::new(1.0, -1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(-1.0, 1.0, 0.0),
            FORWARD,
            Vec3::new(1.0, 1.0, 1.0),
        );
        self.flush();
    }

    pub fn swap(&mut self, window: &Window) {
        self.flush();
        window.gl_swap_window();
    }

    pub fn set_model_matrix(&mut self, model: &Mat4) {
        self.flush();
        self.shader.set_uniform("Model", model).unwrap();
    }
}
