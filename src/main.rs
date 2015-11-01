#[macro_use]
extern crate glium;
extern crate sphere;

use std::collections::{HashMap, LinkedList};
use sphere::gfx::renderable::{Renderable, DefaultRenderer, DynamicUniforms};
use sphere::gfx::simple_shapes::triangle::Triangle;

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium::uniforms::UniformValue::*;
    use std::f32::consts::PI;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Sphere".to_string())
        .with_dimensions(800, 600)
        .build_glium().unwrap();

    let mut renderables: LinkedList<Box<Renderable>> = LinkedList::new();
    renderables.push_back(Box::new(Triangle::new(&display)));

    let mut t: f32 = 0.0;
    loop {
        t += 0.02;
        if t > (2.0*PI) {
            t = 0.0;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let mut uniforms = DynamicUniforms {
            data: HashMap::new()
        };

        uniforms.data.insert("model",
            Mat4([
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]));

        {
            let mut renderer = DefaultRenderer::new(&mut target, uniforms);

            for renderable in renderables.iter_mut() {
                renderable.render(&mut renderer);
            }
        }

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
