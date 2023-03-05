use super::*;

pub struct Renderer {
    _gl_context: GLContext,
    immediate_vertices: Vec<Vertex>,
    vbo: Buffer,
    vao: VertexArray,
    shader: Shader,
    screen_space_shader: Shader,
    width: u32,
    height: u32,

    default_texture: Texture,
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
        let (width, height) = window.size();

        let mut default_texture = Texture::new();
        let data = [255u8; 4];
        default_texture.set_data(1, 1, gl::RGBA, &data);

        let result = Renderer {
            _gl_context: gl_context,
            immediate_vertices: Vec::<Vertex>::new(),
            vbo: Buffer::new(),
            vao: VertexArray::new(),
            shader: Shader::from_source(
                include_str!("../triangle.vert"),
                include_str!("../triangle.frag"),
            )
            .unwrap(),
            screen_space_shader: Shader::from_source(
                include_str!("../screen_space.vert"),
                include_str!("../screen_space.frag"),
            )
            .unwrap(),
            width,
            height,
            default_texture
        };
        result.render_to_window();
        Ok(result)
    }

    #[inline]
    pub fn enable(what: GLenum) {
        unsafe {
            gl::Enable(what);
        }
    }

    pub fn disable(what: GLenum) {
        unsafe {
            gl::Disable(what);
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
        (self.width, self.height) = window.size();
    }

    pub fn render_to_window (&self) {
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
        self.set_projection_matrix(self.width, self.height);
    }

    pub fn set_projection_matrix(&self, width: u32, height: u32) {
        self.screen_space_shader.set_uniform("Projection", cgmath::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0)).unwrap();
        self.shader.set_uniform("Projection", cgmath::perspective(
            cgmath::Deg(60.0),
            (width as f32) / (height as f32),
            0.1,
            1000.0,
        )).unwrap();

    }

    pub fn default_texture(&self) {
        self.default_texture.bind();
    }

    #[inline]
    pub fn push_vertex(&mut self, vertex: Vertex) {
        self.immediate_vertices.push(vertex);
    }

    pub fn push_2d_quad(&mut self, x: f32, y: f32, w: f32, h: f32, color: Vec4) {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        self.push_vertex(Vertex { pos: Vec3::new(x, y, 0.0), normal, uv: Vec2::new(0.0, 0.0), color });
        self.push_vertex(Vertex { pos: Vec3::new(x + w, y, 0.0), normal, uv: Vec2::new(1.0, 0.0), color, });
        self.push_vertex(Vertex { pos: Vec3::new(x + w, y + h, 0.0), normal, uv: Vec2::new(1.0, 1.0), color, });
        self.push_vertex(Vertex { pos: Vec3::new(x, y, 0.0), normal, uv: Vec2::new(0.0, 0.0), color, });
        self.push_vertex(Vertex { pos: Vec3::new(x + w, y + h, 0.0), normal, uv: Vec2::new(1.0, 1.0), color, });
        self.push_vertex(Vertex { pos: Vec3::new(x, y + h, 0.0), normal, uv: Vec2::new(0.0, 1.0), color, });
    }

    pub fn push_2d_sprite(&mut self, p0: Vec2, p1: Vec2, uv0: Vec2, uv1: Vec2) {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let color = Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        self.push_vertex(Vertex { pos: Vec3::new(p0.x, p0.y, 0.0), normal, uv: uv0, color});
        self.push_vertex(Vertex { pos: Vec3::new(p1.x, p0.y, 0.0), normal, uv: Vec2::new(uv1.x, uv0.y), color});
        self.push_vertex(Vertex { pos: Vec3::new(p1.x, p1.y, 0.0), normal, uv: uv1, color});
        self.push_vertex(Vertex { pos: Vec3::new(p0.x, p0.y, 0.0), normal, uv: uv0, color});
        self.push_vertex(Vertex { pos: Vec3::new(p1.x, p1.y, 0.0), normal, uv: uv1, color});
        self.push_vertex(Vertex { pos: Vec3::new(p0.x, p1.y, 0.0), normal, uv: Vec2::new(uv0.x, uv1.y), color});
    }

    pub fn push_quad_corners( &mut self, p0: Point3, p1: Point3, p2: Point3, p3: Point3, normal: Vec3, color: Vec4,) {
        self.push_vertex(Vertex { pos: p0.vec3(), normal, uv: Vec2::new(0.0, 0.0), color, });
        self.push_vertex(Vertex { pos: p1.vec3(), normal, uv: Vec2::new(1.0, 0.0), color, });
        self.push_vertex(Vertex { pos: p2.vec3(), normal, uv: Vec2::new(1.0, 1.0), color, });
        self.push_vertex(Vertex { pos: p0.vec3(), normal, uv: Vec2::new(0.0, 0.0), color, });
        self.push_vertex(Vertex { pos: p2.vec3(), normal, uv: Vec2::new(1.0, 1.0), color, });
        self.push_vertex(Vertex { pos: p3.vec3(), normal, uv: Vec2::new(0.0, 1.0), color, });
    }

    pub fn push_cube(&mut self, color: Vec4) {
        self.push_quad_corners(
            Point3 { x: -0.5, y: -0.5, z: -0.5, },
            Point3 { x: 0.5, y: -0.5, z: -0.5, },
            Point3 { x: 0.5, y: -0.5, z: 0.5, },
            Point3 { x: -0.5, y: -0.5, z: 0.5, },
            UP,
            color,
        ); // -Y
        self.push_quad_corners(
            Point3 { x: -0.5, y: 0.5, z: -0.5, },
            Point3 { x: 0.5, y: 0.5, z: -0.5, },
            Point3 { x: 0.5, y: 0.5, z: 0.5, },
            Point3 { x: -0.5, y: 0.5, z: 0.5, },
            DOWN,
            color,
        ); // +Y
        self.push_quad_corners(
            Point3 { x: -0.5, y: -0.5, z: -0.5, },
            Point3 { x: -0.5, y: 0.5, z: -0.5, },
            Point3 { x: -0.5, y: 0.5, z: 0.5, },
            Point3 { x: -0.5, y: -0.5, z: 0.5, },
            LEFT,
            color,
        ); // -X
        self.push_quad_corners(
            Point3 { x: 0.5, y: -0.5, z: -0.5, },
            Point3 { x: 0.5, y: 0.5, z: -0.5, },
            Point3 { x: 0.5, y: 0.5, z: 0.5, },
            Point3 { x: 0.5, y: -0.5, z: 0.5, },
            RIGHT,
            color,
        ); // +X
        self.push_quad_corners(
            Point3 { z: -0.5, y: -0.5, x: -0.5, },
            Point3 { z: -0.5, y: 0.5, x: -0.5, },
            Point3 { z: -0.5, y: 0.5, x: 0.5, },
            Point3 { z: -0.5, y: -0.5, x: 0.5, },
            BACK,
            color,
        ); // -Z
        self.push_quad_corners(
            Point3 { z: 0.5, y: -0.5, x: -0.5, },
            Point3 { z: 0.5, y: 0.5, x: -0.5, },
            Point3 { z: 0.5, y: 0.5, x: 0.5, },
            Point3 { z: 0.5, y: -0.5, x: 0.5, },
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
            Vec4::new(1.0, 1.0, 1.0, 1.0),
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

    pub fn begin_3d(&mut self, view: Mat4) {
        Renderer::clear(cgmath::Vector4 {
            x: 0.1,
            y: 0.0,
            z: 0.1,
            w: 1.0,
        });
        self.shader.set_used();
        self.shader.set_uniform("View", view).unwrap();

        Renderer::enable(gl::DEPTH_TEST);
    }

    pub fn begin_2d(&mut self) {
        self.flush();
        self.screen_space_shader.set_used();
        Renderer::disable(gl::DEPTH_TEST);
        Renderer::enable(gl::BLEND);
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
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
