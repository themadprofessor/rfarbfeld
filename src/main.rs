#[macro_use] extern crate glium;
extern crate ruff;
extern crate exit_code;

use std::env;
use std::io;

use glium::{Surface, DisplayBuild};
use glium::glutin::Event;

use ruff::Farbfeld;
use ruff::error as rufferr;

#[macro_use] mod macros;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let img = load_img();

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Farbfeld Viewer")
        .with_vsync()
        .build_glium()
        .unwrap_or_else(|err| exit(exit_code::SERVICE_UNAVAILABLE,
                                   format!("Failed to initalise OpenGL! {}", err)));
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
        .unwrap_or_else(|err| exit(exit_code::FAILURE,
                                   format!("Failed to load vertex data for rendering! {}", err)));
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          index_data)
        .unwrap_or_else(|err| exit(exit_code::FAILURE,
                                   format!("Failed to load index data for rendering! {}", err)));
    let dimensions = (*img.width(), *img.height());
    let mut raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.pixels()
                                                                             .iter()
                                                                             .flat_map(|pixel| pixel.into_iter())
                                                                             .collect::<Vec<u16>>(),
                                                                         dimensions);
    raw_img.format = glium::texture::ClientFormat::U16U16U16U16; //Defaults to U8U8U8U8 which panics
    let texture = glium::texture::Texture2d::new(&display, raw_img)
        .unwrap_or_else(|err| exit(exit_code::FAILURE,
                                   format!("Failed to convert image for OpenGL! {}", err)));
    let uniform = uniform!(tex: &texture);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertices, &indices, &program, &uniform, &Default::default())
            .unwrap_or_else(|err| exit(exit_code::FAILURE,
                                       format!("Failed to draw image! {}", err)));
        target.finish().unwrap_or_else(|err| exit(exit_code::FAILURE,
                                                  format!("Failed to draw image! {}", err)));

        for event in display.poll_events() {
            if let Event::Closed = event {
                return
            }
        }
    }
}

fn exit<T: AsRef<str>>(code: i32, msg: T) -> ! {
    use std::io::Write;
    if writeln!(std::io::stderr(), "{}", msg.as_ref()).is_err() {
        println!("Failed to write to stderr! {}", msg.as_ref());
    }
    std::process::exit(code)
}

fn load_img() -> Farbfeld {
    if let Some(path) = env::args().nth(1) {
        Farbfeld::from_file(path)
            .unwrap_or_else(|err| handle_load_err("Failed to load from file! ", &err))
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        Farbfeld::from_read(handle)
            .unwrap_or_else(|err| handle_load_err("Failed to read image from stdin! ", &err))
    }
}

fn handle_load_err<T: AsRef<str>>(start: T, err: &rufferr::Error) -> ! {
    match *err.kind() {
        rufferr::ErrorKind::IoError(ref e) =>
            exit(exit_code::IO_ERROR, string_build!(start.as_ref(), &e.to_string())),
        rufferr::ErrorKind::InvalidFarbfeldDimensions =>
            exit(exit_code::DATA_ERROR, string_build!(start.as_ref(), &err.to_string())),
        rufferr::ErrorKind::NomError(ref e) =>
            exit(exit_code::DATA_ERROR, string_build!(start.as_ref(), &e.to_string())),
        _ => unreachable!()
    }
}
