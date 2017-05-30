#[macro_use]
extern crate glium;

#[path = "./tuto-07-teapot.rs"]
mod teapot;

use glium::DisplayBuild;
use glium::Surface;
use glium::Frame;
    
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;    

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 matrix;

    void main() {     
        v_normal = transpose(inverse(mat3(matrix))) * normal;   
        gl_Position = perspective * matrix * vec4(position, 1.0);
    }
"#;
const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140
        
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

fn main() {
    let display = glium::glutin::WindowBuilder::new()    
    .with_depth_buffer(24)
    .build_glium()
    .unwrap();    
    
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
                [0.0, 0.0, 2.0, 1.0_f32], 
            ];        
            
    let light = [-1.0, 0.4, 0.9f32]; // the direction of the light
    
    loop {
        
        calc_t_and_mart(&mut t, &mut matr, 0.0);
        
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let p = get_perspective(&target);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        let uniforms = uniform! {
            matrix: matr,
            u_light: light,
            perspective: p,
        } ;

        target.draw((&positions, &normals), &indices, &program, 
            &uniforms, &params).unwrap();

        target.finish().unwrap();
        
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}


fn calc_t_and_mart(t: &mut f32, matr: &mut [[f32; 4]; 4], val: f32) {
    *t += 0.0002;
    if *t > 0.5 {
        *t = -0.5;
    }
    matr[3][0] = val;
} 


fn get_perspective(target: &Frame) -> [[f32; 4]; 4] {    
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}