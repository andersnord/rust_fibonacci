#[macro_use]
extern crate glium;

struct Player {
    health: i32,
    x: f32,
    y: f32
}

impl Player {
    fn new(health: i32, x: f32, y: f32) -> Player {
        Player {
            health: health,
            x: x,
            y: y
        }
    }
    
    fn handle_input(&self) {
        
    }   
}


fn main() {
    use glium::{DisplayBuild, Surface};
    let display =glium:: glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.1, -0.1] };
    let vertex2 = Vertex { position: [ 0.0,  0.1] };
    let vertex3 = Vertex { position: [ 0.1, -0.1] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    //Create player
    let mut player = Player::new (10, 0.0, 0.0);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;      // our new attribute

        uniform mat4 matrix;

        void main() {
            my_attr = position;     // we need to set the value of each `out` variable.
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;

        void main() {
            color = vec4(my_attr, 0.0, 1.0);   // we build a vec4 from a vec2 and two floats
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = -0.5;

    // Main game loop
    loop {
        
        // PSEUDO
        // get input
        // update player position according to input
        
        player.handle_input();
        
        // we update `t`
        update_position(&mut t);

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , t, 0.0, 1.0f32],
            ]
        };

        














        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        // Check for closed window event.
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

fn update_position(t: &mut f32) -> () {
    *t += 0.00002;
    if *t > 0.5 {
        *t = -0.5;
    }
}