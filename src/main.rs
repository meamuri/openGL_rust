#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;
    
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;

    void main() {
        vec2 pos = position;        
        gl_Position = matrix*vec4(pos, 0.0, 1.0);
    }
"#;
const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.33, 0.0, 0.33, 1.0);
        }
    "#;

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let (v1, v2, v3) = get_triangle();
    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let mut t: f32 = -0.5;
    
    loop {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t  , 0.0, 0.0, 1.0_f32],
            ]
        } ;
        
        let mut target = display.draw();
        target.clear_color(0.0, 0.011, 0.011, 1.0);

        target.draw(&vertex_buffer, &indices, &program, 
            &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();
        
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}

fn get_triangle() -> (Vertex, Vertex, Vertex) {
    let v1 = Vertex { position: [-0.5, -0.5]    };
    let v2 = Vertex { position: [ 0.0, 0.5]     };
    let v3 = Vertex { position: [ 0.5, -0.25]   };
    (v1, v2, v3)
}
