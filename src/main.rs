extern crate image;
extern crate rand;
extern crate rand_distr;
extern crate rayon;

pub mod ray_resolver;
pub mod ray_marcher;
pub mod renderer;
pub mod basic_renderer;
pub mod utilities;
pub mod path_tracer;

use crate::renderer::Renderer;
use image::{ImageBuffer};
use rayon::prelude::*;
use utilities::{SceneData, Vector3};
use ray_resolver::RayResolver;

const WIDTH:u32 = 640;
const HEIGHT:u32 = 480;

const WIDTH_F:f32 = WIDTH as f32 / 2f32;
const HEIGHT_F:f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO:f32 = WIDTH_F / HEIGHT_F;

const PIXELS: u32 = WIDTH * HEIGHT;

enum Renderers{
    BASIC_RENDERER,
    PATH_TRACER
}

const RENDERER: Renderers = Renderers::PATH_TRACER;

fn main() {
    let marcher = ray_marcher::RayMarcher{
        max_distance: 20f32,
        max_steps: 500,
        epsilon: 0.0002f32
    };
    let scene = utilities::SceneData{

    };
    match RENDERER{
        Renderers::BASIC_RENDERER => {
            let renderer = basic_renderer::BasicRenderer{
                resolver: marcher,
            };
            save_render(&renderer, &scene, "image.png");
        }
        Renderers::PATH_TRACER => {
            let renderer = path_tracer::PathTracer{
                resolver: marcher,
                bounces: 4,
                samples: 200,
                epsilon: 0.0002f32,
                contrast: 1f32/5f32,
                brightness: -0.5
            };
            save_render(&renderer, &scene, "image.png");
        }
    }

}

fn save_render<T: Renderer<J>, J: RayResolver>(renderer: &T, scene: &SceneData, file_name: &str) where T: std::marker::Sync{

    let pixels: Vec<u8> = (0..PIXELS).into_par_iter().map(|i| {
        let x = i % WIDTH;
        let y = i / WIDTH;
        let start = utilities::Vector3::new(0f32, 0f32, 0f32);
        let _x: f32 = (x as f32/WIDTH_F-1f32)*ASPECT_RATIO;
        let _y: f32 = -(y as f32/HEIGHT_F-1f32);
        let dir = utilities::Vector3::new(_x, _y, 1f32).normalized();
        let color = renderer.render(start, dir, scene.clone());
        color.to_color_array().to_vec()
    }).flatten().collect();
    let image: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::from_vec(WIDTH, HEIGHT, pixels).unwrap();
    image.save(file_name).unwrap();
}