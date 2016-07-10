//! Using piston2d-graphics with the piston2d-glium_graphics backend from glium,
//! with a context and window created by glium, to draw lines.

extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate viewport;

use std::thread;
use std::time::Duration;

use glium_graphics::{
    Glium2d, OpenGL
};

fn main() {
    use glium::{DisplayBuild, Surface };
    use glium::backend::Facade;

    let (w, h) = (300_u32, 300_u32);

    // have to specify viewport manually
    let viewport = viewport::Viewport {
        rect: [0, 0, w as i32, h as i32],
        draw_size: [w, h],
        window_size: [w, h]
    };

    let display = glium::glutin::WindowBuilder::new().with_dimensions(w, h).build_glium().unwrap();

    // Obtain version from glium Context
    let opengl = {
        let version = display.get_context().get_opengl_version();
        match (version.1, version.2) {
            (2, 0) => OpenGL::V2_0,
            (2, 1) => OpenGL::V2_1,
            (3, 0) => OpenGL::V3_0,
            (3, 1) => OpenGL::V3_1,
            (3, 2) => OpenGL::V3_2,
            (3, 3) => OpenGL::V3_3,
            (4, 0) => OpenGL::V4_0,
            (4, 1) => OpenGL::V4_1,
            (4, 2) => OpenGL::V4_2,
            (4, 3) => OpenGL::V4_3,
            (4, 4) => OpenGL::V4_4,
            (4, 5) => OpenGL::V4_5,
            _ => {
                panic!("unsupported OpenGL version?: not present in glium_graphics::OpenGL enum");
            }
        }
    };
    let mut g2d = Glium2d::new(opengl, &display);

    // We render a static image in this example, so only refresh when needed, and sleep in the meantime.
    loop {
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Refresh => {
                    let mut target = display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 1.0);
                    g2d.draw(&mut target, viewport, |c, g| {
                        use graphics::*;
                        line([1.0, 0.0, 0.0, 1.0], // the red line
                             2.0,
                             [0.0, 0.0, 100.0, 100.0],
                             c.transform, g);
                    });
                    target.finish().unwrap();
                },
                _ => ()
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
