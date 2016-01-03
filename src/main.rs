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
    current_length: f32,
    final_length: f32,
    start_x: f32,
    start_y: f32,
    direction: Direction,
    verticies: Vec<Vertex>,
    transformation_matrix: [[f32; 4]; 4]
}

impl Line {
    fn new(current_length: f32, 
            final_length: f32, 
            start_x: f32, 
            start_y: f32, 
            direction: Direction, 
            verticies: Vec<Vertex>, 
            transformation_matrix: [[f32; 4]; 4] ) 
            -> Line {
        Line {
            current_length: current_length,
            final_length: final_length,
            start_x: start_x,
            start_y: start_y,
            direction: direction,
            verticies: verticies,
            transformation_matrix: transformation_matrix
        }
    }
   
    fn update(&mut self) {
        self.current_length += 0.0008; // Arbitrary update speed.
        self.update_matrix();
    }   
    
    fn update_matrix(&mut self) {
        let line_thickness = 0.025; 
        
        match self.direction {
            Direction::Left => 
                self.transformation_matrix = [
                    [-self.current_length, 0.0, 0.0, 0.0],
                    [0.0, line_thickness, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ],
            Direction::Up => 
                self.transformation_matrix = [
                    [line_thickness, 0.0, 0.0, 0.0],
                    [0.0, self.current_length, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ], 
            Direction::Right => 
                self.transformation_matrix = [
                    [self.current_length, 0.0, 0.0, 0.0],
                    [0.0, line_thickness, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [self.start_x, self.start_y, 0.0, 1.0],
                ],
            Direction::Down => 
                self.transformation_matrix = [
                    [line_thickness, 0.0, 0.0, 0.0],
                    [0.0, -self.current_length, 0.0, 0.0],
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

    let vertex1 = Vertex { position: [ 0.0, 1.0] };
    let vertex2 = Vertex { position: [ 0.0, 0.0] };
    let vertex3 = Vertex { position: [ 1.0, 1.0] };
    let vertex4 = Vertex { position: [ 1.0, 0.0] };
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

    // Initialize.
    let length = 0.1;
    let mut direction = Direction::Down;
    let first_line = Line::new(0.0, length, 0.0, 0.0, direction, shape, matrix);
    let mut lines = vec![ first_line ];

    // Render loop.
    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        
        // Update current line.
        let top_index = lines.len() - 1;
        lines[ top_index ].update();

        // Draw all lines.
        for line in &mut lines {
            let uniforms = uniform! {
                matrix: line.transformation_matrix
            };
             
            target.draw(&vertex_buffer, 
                        &indices, 
                        &program, 
                        &uniforms, 
                        &Default::default())
                        .unwrap();
        }
        
        target.finish().unwrap();

        // Check length for current line.
        if lines[ top_index ].current_length > lines[ top_index ].final_length {
            // Calculate fibonacci.
            let mut new_length = lines[ top_index ].final_length;
            if lines.len() > 2 {
                new_length = lines[ top_index ].final_length + (lines[ top_index - 1 ].final_length / 5.0); //Arbitrary value to reduce growth.
            }
            
            // Calculate new start position.
            let prev_start_pos_x = lines[ top_index ].start_x;
            let prev_start_pos_y = lines[ top_index ].start_y;
            let prev_length = lines[ top_index ].final_length;
            let mut new_start_pos_x = 0.0;
            let mut new_start_pos_y = 0.0;
            
            {
                let prev_direction = & lines[ top_index ].direction;
                match *prev_direction {
                    Direction::Left => {
                            // DOWN
                            new_start_pos_x = prev_start_pos_x - prev_length;
                            new_start_pos_y = prev_start_pos_y;
                        }
                    Direction::Up => {
                            // LEFT
                            new_start_pos_x = prev_start_pos_x;
                            new_start_pos_y = prev_start_pos_y + prev_length;
                        }
                    Direction::Right => {
                            // UP
                            new_start_pos_x = prev_start_pos_x + prev_length;
                            new_start_pos_y = prev_start_pos_y;
                        }
                    Direction::Down => {
                            // RIGHT
                            new_start_pos_x = prev_start_pos_x;
                            new_start_pos_y = prev_start_pos_y - prev_length;
                        }
                } 
            }
            
            // Get new direction.
            direction = select_new_direction(&mut lines[ top_index ].direction);
            
            let new_line = Line::new(0.0, new_length, new_start_pos_x, new_start_pos_y, direction, lines[ top_index ].verticies.clone(), matrix);
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

fn select_new_direction(direction: &mut Direction) -> Direction {
    match *direction {
        Direction::Left => 
            return Direction::Down,
        Direction::Up => 
            return Direction::Left,
        Direction::Right => 
            return Direction::Up,
        Direction::Down => 
            return Direction::Right,
    }
}