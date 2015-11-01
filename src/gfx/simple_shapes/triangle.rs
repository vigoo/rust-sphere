use glium;
use gfx::renderable::{Renderer, Renderable};
use gfx::vertices::Vertex2D;

pub struct Triangle {
    vertices: glium::VertexBuffer<Vertex2D>,
    indices: glium::index::NoIndices,
    program: glium::program::Program
}

impl Renderable for Triangle {
    fn render(&self, renderer: &mut Renderer) {
        renderer.render_vertices_2d(&self.vertices, &self.indices, &self.program);
    }
}

fn triangle_vertex_buffer<F>(display: &F) -> glium::VertexBuffer<Vertex2D>
    where F: glium::backend::Facade {

    let vertices = vec![
        Vertex2D { position: [ -0.5,  0.0], tex_coords: [0.0, 0.0] },
        Vertex2D { position: [  0.5,  0.0], tex_coords: [1.0, 0.0] },
        Vertex2D { position: [  0.0,  0.5], tex_coords: [0.5, 1.0] }
    ];

    return glium::VertexBuffer::new(display, &vertices).unwrap();
}

impl Triangle {
    pub fn new<F: glium::backend::Facade>(display: &F) -> Triangle {

            let vertex_shader_src = r#"
                    #version 140

                    in vec2 position;
                    in vec2 tex_coords;

                    out vec2 colordef;
                    out vec2 v_tex_coords;

                    uniform mat4 model;

                    void main() {
                        colordef = position;
                        v_tex_coords = tex_coords;
                        gl_Position = model * vec4(position, 0.0, 1.0);
                    }
                "#;

            let fragment_shader_src = r#"
                    #version 140

                    in vec2 colordef;
                    in vec2 v_tex_coords;
                    out vec4 color;

                    void main() {
                        if (mod(floor(v_tex_coords.x * 10), 2.0) == mod(floor(v_tex_coords.y * 10), 2.0))
                            color = vec4(1.0, 0.0, 0.0, 0.0);
                        else
                            color = vec4(1.0, 1.0, gl_FragCoord.y/600.0, 0.0);
                    }
                "#;

            Triangle {
                vertices: triangle_vertex_buffer(display),
                indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                program: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
            }
    }
}
