extern crate image;
extern crate rand;
extern crate rand_distr;
extern crate rayon;
extern crate indicatif;

pub mod ray_resolver;
pub mod ray_marcher;
pub mod renderer;
pub mod basic_renderer;
pub mod utilities;
pub mod path_tracer;

use crate::renderer::Renderer;
use image::{ImageBuffer};
use rayon::prelude::*;
use utilities::{Vector3, SceneData};
use ray_resolver::RayResolver;
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressStyle};

const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;

const WIDTH_F:f32 = WIDTH as f32 / 2f32;
const HEIGHT_F:f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO:f32 = WIDTH_F / HEIGHT_F;

const PIXELS: u32 = WIDTH * HEIGHT;

enum Renderers{
    BasicRenderer,
    PathTracer
}

const RENDERER: Renderers = Renderers::PathTracer;


fn main() {
    let resolver = ray_marcher::RayMarcher{
        max_distance: 20f32,
        max_steps: 500,
        epsilon: 0.0002f32
    };
    let scene = utilities::SceneData{
        camera_position: Vector3::new(0f32, 0f32, 0f32),
        camera_target: Vector3::new(0f32, 0f32, 1f32)
    };
    match RENDERER{
        Renderers::BasicRenderer => {
            let renderer = basic_renderer::BasicRenderer{
                resolver: resolver,
            };
            save_render(&renderer, &scene, "image.png");
        }
        Renderers::PathTracer => {
            let renderer = path_tracer::PathTracer{
                resolver: resolver,
                bounces: 6,
                samples: 5000,
                epsilon: 0.0002f32,
                contrast: 1f32/5f32,
                brightness: -0.5
            };
            save_render(&renderer, &scene, "image.png");
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
    let pixels: Vec<u8> = (0..PIXELS).into_par_iter().progress_with(bar).map(|i| {
        let x = i % WIDTH;
        let y = i / WIDTH;
        let start = scene.camera_position;
        let _x: f32 = (x as f32/WIDTH_F-1f32)*ASPECT_RATIO;
        let _y: f32 = -(y as f32/HEIGHT_F-1f32);
        let ray_dir = scene.get_look_matrix().multiply(utilities::Vector3::new(_x, _y, 1f32).normalized());
        let color = renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT);
        color.to_color_array().to_vec()
    }).flatten().collect();
    let image: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::from_vec(WIDTH, HEIGHT, pixels).unwrap();
    println!("\n\nWriting to {}", file_name);
    image.save(file_name).unwrap();
}

