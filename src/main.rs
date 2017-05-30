#[macro_use]
extern crate glium;

#[path = "./tuto-07-teapot.rs"]
mod teapot;

use glium::DisplayBuild;
use glium::Surface;
    
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    out vec4 my_attr;

    uniform mat4 matrix;

    void main() {
        my_attr = vec4(position, 1.0);
        gl_Position = matrix*vec4(position, 1.0);
    }
"#;
const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140

        in vec4 my_attr;
        out vec4 color;

        void main() {
            color = my_attr;
        }
    "#;

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
    
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();

    let mut t: f32 = -0.5;
    let mut matr = [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0_f32], 
            ];        
    loop {
        
        calc_t_and_mart(&mut t, &mut matr, 0.0);

        let uniforms = uniform! {
            matrix: matr,
        } ;
        
        let mut target = display.draw();
        target.clear_color(0.0, 0.011, 0.011, 1.0);

        target.draw((&positions, &normals), &indices, &program, 
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

fn calc_t_and_mart(t: &mut f32, matr: &mut [[f32; 4]; 4], val: f32) {
    *t += 0.0002;
    if *t > 0.5 {
        *t = -0.5;
    }
    matr[3][0] = val;
} 
