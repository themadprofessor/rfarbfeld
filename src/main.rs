#[macro_use] extern crate glium;
extern crate ruff;

use std::env;
use std::io;

use glium::{Surface, DisplayBuild};
use glium::glutin::Event;

use ruff::Farbfeld;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let img = if let Some(path) = env::args().nth(1) {
        Farbfeld::from_file(path)
            .unwrap_or_else(|err| panic!(format!("Failed to load image from file! {}", err)))
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        Farbfeld::from_read(handle).expect("Failed to read load image from stdin!")
    };

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Farbfeld Viewer")
        .with_vsync()
        .build_glium()
        .expect("Failed to initalise OpenGL!");
    let program = glium::program::Program::from_source(&display, r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 uv;

        void main() {
            uv = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }"#, r#"
        #version 140
        in vec2 uv;
        out vec4 color;
        uniform sampler2D tex;

        void main() {
            color = texture(tex, uv);
        }"#, None).expect("Failed to create shader program!");

    let data = &[Vertex{position: [-1.0, 1.0], tex_coords: [0.0,1.0]},
        Vertex{position: [1.0, 1.0], tex_coords: [1.0,1.0]},
        Vertex{position: [1.0, -1.0], tex_coords: [1.0,0.0]},
        Vertex{position: [-1.0, -1.0], tex_coords: [0.0,0.0]}];
    let index_data: &[u16; 6] = &[0, 1, 2, 2, 3, 0];
    let vertices = glium::VertexBuffer::new(&display, data)
        .expect("Failed to load vertex data for rendering!");
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList, index_data)
        .expect("Failed to load index data for rendering!");
    let dimensions = (*img.width(), *img.height());
    let mut raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.pixels()
                                                                             .iter()
                                                                             .flat_map(|pixel| pixel.into_iter())
                                                                             .collect::<Vec<u16>>(),
                                                                         dimensions);
    raw_img.format = glium::texture::ClientFormat::U16U16U16U16; //Defaults to U8U8U8U8 which panics
    let texture = glium::texture::Texture2d::new(&display, raw_img)
        .expect("Failed to convert image for OpenGL!");
    let uniform = uniform!(tex: &texture);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertices, &indices, &program, &uniform, &Default::default())
            .expect("Failed to draw image!");
        target.finish().expect("Failed to draw image!");

        for event in display.poll_events() {
            match event {
                Event::Closed => {return}
                _ => ()
            }
        }
    }
}

