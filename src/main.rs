extern crate exr;
extern crate image;
extern crate indicatif;
extern crate rayon;

pub mod error;
pub mod ray_resolvers;
pub mod renderers;
mod scene;
pub mod utilities;

use crate::renderers::renderer::Renderer;
use image::ImageBuffer;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use ray_resolvers::ray_resolver::RayResolver;
use rayon::prelude::*;
use renderers::basic_renderer;
use scene::get_resolver;
use utilities::{SceneData, Vector3};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const WIDTH_F: f32 = WIDTH as f32 / 2f32;
const HEIGHT_F: f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO: f32 = WIDTH_F / HEIGHT_F;

const PIXELS: u32 = WIDTH * HEIGHT;
const FOV: f32 = 1.2;

const FILE_NAME: &str = "image.png";


fn main() {
    let resolver = get_resolver();
    let scene = utilities::SceneData {
        camera_position: Vector3::new(0.0, 4.0, -5.0),
        camera_target: Vector3::new(0f32, 2.0f32, 0f32),
        fog_amount: 50.0,
        fog: false,
    };
    let renderer = basic_renderer::BasicRenderer { resolver };
    save_render(&renderer, &scene, FILE_NAME);
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

fn render_pixel<T: Renderer<J>, J: RayResolver>(
    renderer: &T,
    scene: &SceneData,
    x: u32,
    y: u32,
) -> Vector3 {
    let start = scene.camera_position;
    let _x: f32 = (x as f32 / WIDTH_F - 1f32) * ASPECT_RATIO;
    let _y: f32 = -(y as f32 / HEIGHT_F - 1f32);
    let ray_dir = scene
        .get_look_matrix()
        .multiply(utilities::Vector3::new(_x, _y, FOV).normalized());
    renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT)
}
