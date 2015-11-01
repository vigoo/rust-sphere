use glium;
use glium::{Frame, Surface};
use glium::uniforms::{Uniforms, UniformValue};
use gfx::vertices::Vertex2D;
use std::collections::HashMap;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}

pub trait Renderer {
    fn render_vertices_2d(&mut self, vertices: &glium::VertexBuffer<Vertex2D>, indices: &glium::index::NoIndices, program: &glium::program::Program);
}

pub struct DynamicUniforms<'a> {
    pub data: HashMap<&'a str, glium::uniforms::UniformValue<'a>>
}

impl<'a> Uniforms for DynamicUniforms<'a> {
    fn visit_values<'b, F: FnMut(&str, UniformValue<'b>)>(&'b self, mut f: F) {
        for (k, v) in self.data.iter() {
            f(k, *v)
        }
    }
}

pub struct DefaultRenderer<'a> {
    surface: &'a mut Frame,
    uniforms: DynamicUniforms<'a>
}

impl<'a> DefaultRenderer<'a> {
    pub fn new(frame: &'a mut Frame, uniforms: DynamicUniforms<'a>) -> DefaultRenderer<'a> {
        DefaultRenderer {
            surface: frame,
            uniforms: uniforms
        }
    }
}

impl<'a> Renderer for DefaultRenderer<'a> {
    fn render_vertices_2d(&mut self, vertices: &glium::VertexBuffer<Vertex2D>, indices: &glium::index::NoIndices, program: &glium::program::Program) {
        self.surface.draw(vertices, indices, program, &self.uniforms, &Default::default()).unwrap()
    }
}
