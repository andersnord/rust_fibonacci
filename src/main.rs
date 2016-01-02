// TODO:
// Update Line w direction o vilken nr det är så man vet längd.

#[macro_use]
extern crate glium;

enum Direction {
    Left,
    Up,
    Right,
    Down    
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

struct Line {
    length: f32,
    start_x: f32,
    start_y: f32,
    direction: Direction,
    verticies: Vec<Vertex>,
    transformation_matrix: [[f32; 4]; 4]
}

impl Line {
    fn new(length: f32, start_x: f32, start_y: f32, direction: Direction, verticies: Vec<Vertex>, transformation_matrix: [[f32; 4]; 4] ) -> Line {
        Line {
            length: length,
            start_x: start_x,
            start_y: start_y,
            direction: direction,
            verticies: verticies,
            transformation_matrix: transformation_matrix
        }
    }
   
    fn update(&mut self) {
        // PSEUDO
        // Update length of "Current line"
        // Update matrix
        // Check direction of the line
        // Check length if long enough for starting new line
        // Create new line if yes and make that one the current line

        self.length += 0.0002; //TODO: use timer
        self.update_matrix();
    }   
    
    fn update_matrix(&mut self)
    {
        match self.direction {
            Direction::Left => 
                self.transformation_matrix = [
                    [-self.length, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ],
            Direction::Up => 
                self.transformation_matrix = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, self.length, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ], 
            Direction::Right => 
                self.transformation_matrix = [
                    [self.length, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ],
            Direction::Down => 
                self.transformation_matrix = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, -self.length, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ],
        }
    }
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display =glium:: glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [ 0.0,  0.1] };
    let vertex2 = Vertex { position: [ 0.0,  0.0] };
    let vertex3 = Vertex { position: [ 0.1,  0.1] };
    let vertex4 = Vertex { position: [ 0.1,  0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

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

    let program = glium::Program::from_source(
        &display, 
        vertex_shader_src, 
        fragment_shader_src, 
        None)
        .unwrap();
    
    let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

    let mut first_line = Line::new(0.0, 0.0, 0.0, Direction::Up, shape, matrix);
    
    let mut lines = vec![ first_line ];

    // Main game loop
    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        
        // Update latest added line
        let top_index = lines.len() - 1;
        lines[ top_index ].update();

        // Draw all lines
        for line in &mut lines {
            let uniforms = uniform! {
                matrix: line.transformation_matrix
            };
             
            target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        }
        
        target.finish().unwrap();

        //Check top line length and if new one is to be added
        if lines[ top_index ].length > 5.5 {
            let mut new_line = Line::new(0.0, 0.0, 0.0, Direction::Left, lines[ top_index ].verticies.clone(), matrix);
            lines.push(new_line)
        }


        // Check for closed window event.
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}