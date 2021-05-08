extern crate exr;
extern crate image;
extern crate indicatif;
extern crate rand;
extern crate rand_distr;
extern crate rayon;

pub mod basic_renderer;
mod config_parser;
pub mod path_tracer;
pub mod ray_marcher;
pub mod ray_resolver;
pub mod renderer;
pub mod utilities;
pub mod bvh;
pub mod error;
mod scene;

use crate::renderer::Renderer;
use exr::{image::{read::specific_channels}, prelude::*};
use image::ImageBuffer;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand_distr::Uniform;
use ray_resolver::RayResolver;
use rayon::prelude::*;
use scene::get_resolver;
use std::{f32::consts::{FRAC_PI_2, PI}, usize};
use utilities::{SceneData, Vector3};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const WIDTH_F: f32 = WIDTH as f32 / 2f32;
const HEIGHT_F: f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO: f32 = WIDTH_F / HEIGHT_F;

const PIXELS: u32 = WIDTH * HEIGHT;
const FOV: f32 = 1.2;

enum Renderers {
    BasicRenderer,
    PathTracer,
}

const RENDERER: Renderers = Renderers::PathTracer;

enum OutputFormats {
    PNG,
    OPENEXR,
}

const OUTPUT_FORMAT: OutputFormats = OutputFormats::PNG;
const FILE_NAME: &str = "image.png";

enum CameraTypes {
    Normal,
    Equirectangular,
}

const CAMERA_TYPE: CameraTypes = CameraTypes::Normal;

fn main() {
    let resolver = get_resolver();
    let scene = utilities::SceneData {
        camera_position: Vector3::new(0.0, 4.0, -5.0),
        camera_target: Vector3::new(0f32, 2.0f32, 0f32),
        fog_amount: 50.0,
        fog: false,
    };
    match RENDERER {
        Renderers::BasicRenderer => {
            let renderer = basic_renderer::BasicRenderer { resolver: resolver };
            save_render(&renderer, &scene, FILE_NAME);
        }
        Renderers::PathTracer => {
            let skybox = read().no_deep_data().largest_resolution_level().rgba_channels(|resolution, _|{
                let p = [0.0; 4];
                let line = vec![p; resolution.width()];
                let img = vec![line; resolution.height()];
                img
            }, |img, pos, (r, g, b, a): (f32, f32, f32, f32)|{
                img[pos.y()][pos.x()] = [r, g, b, a];
            }).first_valid_layer().all_attributes().from_file("env.exr").unwrap();
            let pixels: Vec<Vec<[f32; 4]>> = skybox.layer_data.channel_data.pixels;
            let s = (pixels.first().unwrap().len(), pixels.len());
            let renderer = path_tracer::PathTracer {
                resolver: resolver,
                bounces: 5,
                samples: 500,
                epsilon: 0.0002f32,
                contrast: 1f32 / 5f32,
                brightness: -0.5,
                depth_of_field: 4f32,
                dof_distr: Uniform::new(-0.075, 0.075),
                dof: false,
                skybox_size: s,
                skybox: pixels,
            };
            save_render(&renderer, &scene, FILE_NAME);
        }
    }
}

fn save_render<T: Renderer<J>, J: RayResolver>(renderer: &T, scene: &SceneData, file_name: &str)
where
    T: std::marker::Sync,
{
    let style = ProgressStyle::default_bar()
        .template("{prefix}[{wide_bar}] {percent}%")
        .progress_chars("=> ");
    let bar = ProgressBar::new(PIXELS as u64);
    bar.set_draw_delta((PIXELS / 100) as u64);
    bar.set_style(style);
    bar.set_prefix("Rendering... ");
    match OUTPUT_FORMAT {
        OutputFormats::PNG => {
            let pixels: Vec<u8> = (0..PIXELS)
                .into_par_iter()
                .progress_with(bar)
                .map(|i| {
                    let x = i % WIDTH;
                    let y = i / WIDTH;
                    //TODO: better toneing
                    let color = if T::needs_toneing() {
                        let color = render_pixel(renderer, scene, x, y);
                        let color_g = color.pow(1f32 / 2.2f32);
                        let lum = color_g.x * 0.2126 + color_g.y * 0.7152 + color_g.z * 0.0722;
                        let color = color.multiply(2.0 / (lum + 1.0));
                        color.pow(1f32 / 2.2f32)
                    } else {
                        render_pixel(renderer, scene, x, y)
                    };
                    color.to_color_array().to_vec()
                })
                .flatten()
                .collect();
            let image: ImageBuffer<image::Rgb<u8>, _> =
                ImageBuffer::from_vec(WIDTH, HEIGHT, pixels).unwrap();
            println!("\n\nWriting to {}", file_name);
            image.save(file_name).unwrap();
        }
        OutputFormats::OPENEXR => {
            let pixels: Vec<Vector3> = (0..PIXELS)
                .into_par_iter()
                .progress_with(bar)
                .map(|i| {
                    let x = i % WIDTH;
                    let y = i / WIDTH;
                    let color = render_pixel(renderer, scene, x, y);
                    color
                })
                .collect();
            println!("\n\nWriting to {}", file_name);
            let layer = Layer::new((WIDTH as usize, HEIGHT as usize), LayerAttributes::default(), Encoding::SMALL_FAST_LOSSY, SpecificChannels::rgb(|pos: Vec2<usize>|{
                let i = pos.0 + pos.1 * WIDTH as usize;
                let c = pixels[i];
                (c.x, c.y, c.z)
            }));
            let image = Image::from_layer(layer);
            image.write().to_file(file_name).unwrap();
        }
    }
}

fn render_pixel<T: Renderer<J>, J: RayResolver>(
    renderer: &T,
    scene: &SceneData,
    x: u32,
    y: u32,
) -> Vector3 {
    match CAMERA_TYPE {
        CameraTypes::Normal => {
            let start = scene.camera_position;
            let _x: f32 = (x as f32 / WIDTH_F - 1f32) * ASPECT_RATIO;
            let _y: f32 = -(y as f32 / HEIGHT_F - 1f32);
            let ray_dir = scene
                .get_look_matrix()
                .multiply(utilities::Vector3::new(_x, _y, FOV).normalized());
            renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT)
        }
        CameraTypes::Equirectangular => {
            let start = scene.camera_position;
            let clip_x: f32 = x as f32 / WIDTH_F - 1f32;
            let clip_y: f32 = -(y as f32 / HEIGHT_F - 1f32);

            let latitude = clip_y * FRAC_PI_2;
            let longitude = clip_x * PI;

            let (_y, t) = latitude.sin_cos();
            let _z = longitude.cos() * t;
            let _x = longitude.sin() * t;

            let ray_dir = Vector3::new(_x, _y, _z);
            renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT)
        }
    }
}
