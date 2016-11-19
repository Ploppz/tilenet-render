use std::fs::File;
use std::io::Read;
use std::borrow::Cow;


use glium;
use glium::{ Display, Surface };
use glium::texture::{ Texture2d, ClientFormat, RawImage2d };

use tile_net::TileNet;


pub struct Renderer {
    display: Display,
    net_width: usize,
    net_height: usize,

    // OpenGL
    shader_prg: glium::Program,
    quad_vbo: glium::VertexBuffer<Vertex>,
    texture: Texture2d,
}

impl Renderer {
    pub fn new<T>(display: Display, net: &TileNet<T>) -> Renderer
    where T: Clone + glium::texture::PixelValue {

        let vert_src = include_str!("../../shaders/xyuv_tex.vert");
        let frag_src = include_str!("../../shaders/xyuv_tex.frag");
        let shader_prg = glium::Program::from_source(&display, vert_src, frag_src, None).unwrap();
        let fullscreen_quad = vec![ Vertex { pos: [-1.0, -1.0]},
                                    Vertex { pos: [1.0, -1.0]},
                                    Vertex { pos: [1.0, 1.0]},

                                    Vertex { pos: [1.0, 1.0]},
                                    Vertex { pos: [-1.0, 1.0]},
                                    Vertex { pos: [-1.0, -1.0]}];

        let quad_vbo = ::glium::VertexBuffer::new(&display, &fullscreen_quad).unwrap();
        let texture_data: Vec<Vec<u8>> = vec!(vec!(0; net.get_size().0); net.get_size().1);
        let texture = glium::texture::Texture2d::new(&display, texture_data).unwrap();

        let mut new = Renderer {
            display: display,
            net_width: net.get_size().0,
            net_height: net.get_size().1,

            shader_prg: shader_prg,
            quad_vbo: quad_vbo,
            texture: texture,
        };
        new.upload_texture(net);
        new
    }

    pub fn render(&mut self, target: &mut glium::Frame, center_x: f32, center_y: f32, zoom: f32, width: u32, height: u32) {

        let uniforms = uniform! (
            sampler: glium::uniforms::Sampler::new(&self.texture)
                    .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            view_size: [width as f32 / zoom, height as f32 / zoom],
            tex_size: [self.net_width as f32, self.net_height as f32],
            screen_center: [center_x, center_y],
        );
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(self.quad_vbo.slice(0..6).unwrap(), indices, &self.shader_prg, &uniforms, &Default::default()).unwrap();


        // END
    }

    fn upload_texture<T>(&mut self, net: &TileNet<T>)
        where T: Clone + glium::texture::PixelValue {
        let net_size = net.get_size();
        let upload_area = glium::Rect { left: 0, bottom: 0, width: net.get_size().0 as u32, height: net.get_size().1 as u32};
        let upload_data = RawImage2d {
            data: Cow::Borrowed(net.get_raw()),
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
}

implement_vertex!(Vertex, pos);
