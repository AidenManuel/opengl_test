#[macro_use]
extern crate glium;

use glium::Surface;

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
    }
    implement_vertex!(Vertex, position);

    // The vertices which define the triangle being drawn
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    // A buffer to put those vertices in so they can be handed off to the GPU
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // This tells OpenGL that we don't use indices and instead want to draw a certain number of separate triangles.
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float x;

        void main() {
            vec2 pos = position;
            pos.x += x;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
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
                    t += 0.01;
                    // We use the sine of t as an offset, this way we get a nice smooth animation
                    let x_off = t.sin() * 0.5;
                    let r_off = t.cos() * 0.5 + 0.5;
    
                    let mut target = display.draw();
                    target.clear_color(r_off, 0.0, r_off, 1.0);
                    target.draw(&vertex_buffer, &indices, &program, &uniform! { x: x_off },
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
