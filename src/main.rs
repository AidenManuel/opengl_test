#[macro_use]
extern crate glium;

use glium::Surface;

#[path = "../models/tuto-07-teapot.rs"]
mod teapot;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #2")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);
    
    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];

    // A buffer to put those vertices in so they can be handed off to the GPU
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // This tells OpenGL that we don't use indices and instead want to draw a certain number of separate triangles.
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Adding Textures
    let image = image::load(std::io::Cursor::new(&include_bytes!("../images/hmmm.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    
    let mut t: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // We update `t`
                    t += 0.015;
                    // We use the sine of t as an offset, this way we get a nice smooth animation
                    let x = t.sin() * 0.5;
    
                    let mut target = display.draw();
                    target.clear_color(0.85, 0.65, 0.72, 1.0);
                    let uniforms = uniform! {
                        matrix: [
                            [ t.cos(),  t.sin(),    0.0, 0.0],
                            [-t.sin(),  t.cos(),    0.0, 0.0],
                            [0.0,       0.0,        1.0, 0.0],
                            [0.0,       0.0,        0.0, 1.0f32],
                        ],
                        tex: &texture,
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                },
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
