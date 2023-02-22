use super::Vertex;

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
}

impl From<Vec<Vertex>> for Mesh {
    fn from(v: Vec<Vertex>) -> Self {
        Mesh { vertices: v }
    }
}
