extern crate image;
extern crate rand;
extern crate rand_distr;
extern crate rayon;
extern crate indicatif;
extern crate exr;

pub mod ray_resolver;
pub mod ray_marcher;
pub mod renderer;
pub mod basic_renderer;
pub mod utilities;
pub mod path_tracer;
mod config_parser;

use crate::renderer::Renderer;
use image::{ImageBuffer};
use rayon::prelude::*;
use utilities::{Vector3, SceneData};
use ray_resolver::RayResolver;
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressStyle};
use exr::{prelude::*, image::rgba::Channel};
use std::f32::consts::{PI, FRAC_PI_2};
use rand_distr::Uniform;

const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;

const WIDTH_F:f32 = WIDTH as f32 / 2f32;
const HEIGHT_F:f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO:f32 = WIDTH_F / HEIGHT_F;

const PIXELS: u32 = WIDTH * HEIGHT;
const FOV: f32 = 1.2;


enum Renderers{
    BasicRenderer,
    PathTracer,
}

const RENDERER: Renderers = Renderers::PathTracer;

enum OutputFormats{
    PNG,
    OPENEXR
}

const OUTPUT_FORMAT: OutputFormats = OutputFormats::PNG;
const FILE_NAME: &str = "image.png";

enum CameraTypes{
    Normal,
    Equirectangular
}

const CAMERA_TYPE: CameraTypes = CameraTypes::Normal;


fn main() {

    let resolver = ray_marcher::RayMarcher{
        max_distance: 150f32,
        max_steps: 1000,
        epsilon: 0.0002f32
    };
    let scene = utilities::SceneData{
        camera_position: Vector3::new(0f32, 0f32, 0f32),
        camera_target: Vector3::new(0f32, 0f32, 1f32),
        fog_amount: 1000.0,
        fog: true
    };
    match RENDERER{
        Renderers::BasicRenderer => {
            let renderer = basic_renderer::BasicRenderer{
                resolver: resolver,
            };
            save_render(&renderer, &scene, FILE_NAME);
        }
        Renderers::PathTracer => {
            let renderer = path_tracer::PathTracer{
                resolver: resolver,
                bounces: 5,
                samples: 500,
                epsilon: 0.0002f32,
                contrast: 1f32/5f32,
                brightness: -0.5,
                depth_of_field: 3f32,
                dof_distr: Uniform::new(-0.1, 0.1),
                dof: false
            };
            save_render(&renderer, &scene, FILE_NAME);
        }
    }

}

fn save_render<T: Renderer<J>, J: RayResolver>(renderer: &T, scene: &SceneData, file_name: &str) where T: std::marker::Sync{
    let style = ProgressStyle::default_bar()
        .template("{prefix}[{wide_bar}] {percent}%")
        .progress_chars("=> ");
    let bar = ProgressBar::new(PIXELS as u64);
    bar.set_draw_delta((PIXELS/100) as u64);
    bar.set_style(style);
    bar.set_prefix("Rendering... ");
    match OUTPUT_FORMAT{
        OutputFormats::PNG => {
            let pixels: Vec<u8> = (0..PIXELS).into_par_iter().progress_with(bar).map(|i| {
                let x = i % WIDTH;
                let y = i / WIDTH;
                //TODO: better toneing
                let color = if T::needs_toneing(){
                    let color = render_pixel(renderer, scene, x, y);
                    let color_g = color.pow(1f32/2.2f32);
                    let lum = color_g.x * 0.2126 + color_g.y * 0.7152 + color_g.z * 0.0722;
                    let color = color.multiply(2.0/(lum+1.0));
                    color.pow(1f32/2.2f32)
                } else {
                    render_pixel(renderer, scene, x, y)
                };
                color.to_color_array().to_vec()
            }).flatten().collect();
            let image: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::from_vec(WIDTH, HEIGHT, pixels).unwrap();
            println!("\n\nWriting to {}", file_name);
            image.save(file_name).unwrap();
        }
        OutputFormats::OPENEXR => {
            let pixels: Vec<Vector3> = (0..PIXELS).into_par_iter().progress_with(bar).map(|i| {
                let x = i % WIDTH;
                let y = i / WIDTH;
                let color = render_pixel(renderer, scene, x, y);
                color
            }).collect();
            println!("\n\nWriting to {}", file_name);
            let image_info = rgba::ImageInfo::rgb(
                (WIDTH as usize, HEIGHT as usize),
                Channel::linear(SampleType::F32)
            );
            image_info
                .with_encoding(rgba::Encoding::for_compression(Compression::RLE))
                .write_pixels_to_file(file_name, write_options::high(), &|pos: Vec2<usize>|{
                    let i = pos.0 + pos.1 * WIDTH as usize;
                    let c = pixels[i];
                    rgba::Pixel::rgb(c.x, c.y, c.z)
                }).unwrap();
        }
    }

}

fn render_pixel<T: Renderer<J>, J: RayResolver>(renderer: &T, scene: &SceneData, x: u32, y: u32) -> Vector3{
    match CAMERA_TYPE {
        CameraTypes::Normal => {
            let start = scene.camera_position;
            let _x: f32 = (x as f32/WIDTH_F-1f32)*ASPECT_RATIO;
            let _y: f32 = -(y as f32/HEIGHT_F-1f32);
            let ray_dir = scene.get_look_matrix().multiply(utilities::Vector3::new(_x, _y, FOV).normalized());
            renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT)
        }
        CameraTypes::Equirectangular => {
            let start = scene.camera_position;
            let clip_x: f32 = x as f32/WIDTH_F-1f32;
            let clip_y: f32 = -(y as f32/HEIGHT_F-1f32);

            let latitude = clip_y*FRAC_PI_2;
            let longitude = clip_x*PI;

            let (_y, t) = latitude.sin_cos();
            let _z = longitude.cos() * t;
            let _x = longitude.sin() * t;

            let ray_dir = Vector3::new(_x, _y, _z);
            renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT)
        }
    }
    
}