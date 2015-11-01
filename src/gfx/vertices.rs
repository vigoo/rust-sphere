#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2]
}

implement_vertex!(Vertex2D, position, tex_coords);
