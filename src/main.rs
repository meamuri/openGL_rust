#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;
    
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let v1 = Vertex { position: [-0.5, -0.5]    };
    let v2 = Vertex { position: [ 0.0, 0.5]     };
    let v3 = Vertex { position: [ 0.5, -0.25]   };
    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, 
        &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

        target.finish().unwrap();
        
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
