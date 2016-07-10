
extern crate graphics;
#[macro_use] extern crate glium;
extern crate glium_graphics;
extern crate image;
extern crate piston;

use glium::Surface;
use glium_graphics::{
    Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings
};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (300, 300);
    // GliumWindow implements Facade
    let ref mut window: GliumWindow =
        WindowSettings::new("glium test", [w, h])
        .exit_on_esc(true).opengl(opengl).build().unwrap();

    let rust_logo = Texture::from_path(window, "resources/parrot.png",
        Flip::None, &TextureSettings::new()).unwrap();

    // ***************** glium code
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);
    let shape = {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        vec![vertex1, vertex2, vertex3]
    };
    let vertex_buffer = glium::VertexBuffer::new(window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = include_str!("../shaders/pixel.vert");
    let fragment_shader_src = include_str!("../shaders/green_colour.frag");
    let program = glium::Program::from_source(window, vertex_shader_src, fragment_shader_src, None).unwrap();
    // *********************

    let mut g2d = Glium2d::new(opengl, window);
    while let Some(e) = window.next() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let mut target = window.draw();
            g2d.draw(&mut target, args.viewport(), |c, g| {
                clear(color::WHITE, g);
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [0.0, 0.0, 100.0, 100.0],
                          c.transform, g);
                rectangle([0.0, 1.0, 0.0, 0.3],
                          [50.0, 50.0, 100.0, 100.0],
                          c.transform, g);
                image(&rust_logo, c.transform.trans(100.0, 100.0), g);
                let transform = c.transform.flip_v().trans(0.0, -300.0);
                line([0.0, 0.0, 1.0, 1.0], // blue
                     2.0,
                     [0.0, 0.0, 100.0, 100.0],
                     transform, g);
                line([0.0, 1.0, 1.0, 1.0],
                     3.2,
                     [100.0, 100.0, 200.0, 300.0],
                     transform, g);
            });
            // glium draw call - we can just use the Surface (Frame) returned by window.draw()
            // It's convenient to use glium from this backend, because, unlike opengl,
            // glium is stateless - we don't have to worry about state changes made by the backend.
            target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
            target.finish().unwrap();
        }

    }
}
