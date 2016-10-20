use std::fs::File;
use std::io::Read;
use std::borrow::Cow;


use glium;
use glium::{ Display, Surface };
use glium::texture::{ Texture2d, ClientFormat, RawImage2d };

use tile_net;


pub struct TileNet<'a, T> where T: 'a + Clone + glium::texture::PixelValue {
    display: &'a Display,
    net: &'a tile_net::TileNet<T>,

    // OpenGL 
    shader_prg: glium::Program,
    quad_vbo: glium::VertexBuffer<Vertex>,
    texture: Texture2d,
}

impl<'a, T> TileNet<'a, T>
where T: Clone + glium::texture::PixelValue {
    pub fn new(display: &'a Display, net: &'a tile_net::TileNet<T>) -> TileNet<'a, T> {
        let shader_prg = create_program(display, "xyuv_tex");
        let fullscreen_quad = vec![ Vertex { pos: [-1.0, -1.0], texpos: [0.0, 1.0]},
                                    Vertex { pos: [1.0, -1.0],  texpos: [1.0, 1.0]},
                                    Vertex { pos: [1.0, 1.0],   texpos: [1.0, 0.0]},

                                    Vertex { pos: [1.0, 1.0],   texpos: [1.0, 0.0]},
                                    Vertex { pos: [-1.0, 1.0],  texpos: [0.0, 0.0]},
                                    Vertex { pos: [-1.0, -1.0], texpos: [0.0, 1.0]}];

        let quad_vbo = ::glium::VertexBuffer::new(display, &fullscreen_quad).unwrap();
        let texture_data: Vec<Vec<u8>> = vec!(vec!(0; net.get_size().0); net.get_size().1);
        let texture = glium::texture::Texture2d::new(display, texture_data).unwrap();

        let mut new = TileNet {
            display: display,
            net: net,

            shader_prg: shader_prg,
            quad_vbo: quad_vbo,
            texture: texture,
        };
        new.upload_texture();
        new
    }

    pub fn render(&mut self, left: f32, top: f32, width: u32, height: u32) {
        let mut target = self.display.draw();        // target: glium::Frame
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // RENDER 

        let tex_left = left / (self.net.get_size().0 as f32);
        let tex_top = top / (self.net.get_size().1 as f32);
        let tex_width = (width as f32) / (self.net.get_size().0 as f32);
        let tex_height = (height as f32) / (self.net.get_size().1 as f32);

        let uniforms = uniform! (
            sampler: glium::uniforms::Sampler::new(&self.texture).wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
            tex_lefttop: [tex_left, tex_top],
            tex_size: [tex_width, tex_height],
        );
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(self.quad_vbo.slice(0..6).unwrap(), indices, &self.shader_prg, &uniforms, &Default::default()).unwrap();


        // END

        target.finish().unwrap(); 
    }

    fn upload_texture(&mut self) {
        let net_size = self.net.get_size();
        let upload_area = glium::Rect { left: 0, bottom: 0, width: self.net.get_size().0 as u32, height: self.net.get_size().1 as u32};
        let upload_data = RawImage2d {
            data: Cow::Borrowed(self.net.get_raw()),
            width: net_size.0 as u32,
            height: net_size.1 as u32,
            format: ClientFormat::U8,
        };


        self.texture.write(upload_area, upload_data);
    }

}

// For rendering
#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    texpos: [f32; 2],
}

implement_vertex!(Vertex, pos, texpos);


//// Helpers ////
pub fn create_program<F>(display: &F, name: &'static str) -> glium::Program
    where F: glium::backend::Facade
{
    let mut f = File::open("shaders/".to_string() + name + ".vert").unwrap();
    let mut vert_src = String::new();
    let _ = f.read_to_string(&mut vert_src);
    let _ = f = File::open("shaders/".to_string() + name + ".frag").unwrap();
    let mut frag_src = String::new();
    let _ = f.read_to_string(&mut frag_src);

    glium::Program::from_source(display, vert_src.as_str(), frag_src.as_str(), None).unwrap()
}

