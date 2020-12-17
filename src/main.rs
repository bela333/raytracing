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
pub mod snapshot;

use crate::renderer::Renderer;
use image::{ImageBuffer};
use rayon::prelude::*;
use snapshot::Snapshot;
use utilities::{Vector3, SceneData};
use ray_resolver::RayResolver;
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressStyle};
use std::f32::consts::{PI, FRAC_PI_2};
use rand_distr::Uniform;

const WIDTH:u32 = 128;
const HEIGHT:u32 = 128;

const FOV: f32 = 1.2;
const FPS:u32 = 30;
const DURATION:f32 = 6.0;
const SNAPSHOTS: u32 = 6;




const WIDTH_F:f32 = WIDTH as f32 / 2f32;
const HEIGHT_F:f32 = HEIGHT as f32 / 2f32;
const ASPECT_RATIO:f32 = WIDTH_F / HEIGHT_F;
const PIXELS: u32 = WIDTH * HEIGHT;
const SNAPSHOT_DURATION: f32 = DURATION / SNAPSHOTS as f32;


enum Renderers{
    BasicRenderer,
    PathTracer,
}

const RENDERER: Renderers = Renderers::PathTracer;

fn main() {
    let resolver = ray_marcher::RayMarcher{
        max_distance: 150f32,
        max_steps: 1000,
        epsilon: 0.0002f32,
    };

    match RENDERER{
        Renderers::BasicRenderer => {
            let renderer = basic_renderer::BasicRenderer{
                resolver: resolver,
            };
            render_video(&renderer);
        }
        Renderers::PathTracer => {
            let renderer = path_tracer::PathTracer{
                resolver: resolver,
                bounces: 5,
                samples: 100,
                epsilon: 0.0002f32,
                contrast: 1f32/5f32,
                brightness: -0.5,
                depth_of_field: 3f32,
                dof_distr: Uniform::new(-0.1, 0.1),
                dof: false
            };
            render_video(&renderer);
        }
    }
}


fn render_video<T: Renderer<J>, J:RayResolver>(renderer: &T) where T: std::marker::Sync{
    let center_x = 1.0;
    let center_y = 1.0;
    let mut scene = utilities::SceneData{
        camera_position: Vector3::new(center_x, center_y, 0f32),
        camera_target: Vector3::new(1f32, 1f32, 0f32),
        fog_amount: 0.0,
        fog: false,
        snapshot1: Snapshot::read("snapshots/0.bin"),
        snapshot2: Snapshot::read("snapshots/1.bin"),
        transform: 0.0
    };
    let frame_count = (FPS as f32*DURATION) as i32;
    let mut last_id = 0;
    for frame_number in 0..frame_count{
        //let t = frame_number as f32 / FPS as f32;
        let t = 5.5;
        let id = (t/SNAPSHOT_DURATION) as u32;
        if id > last_id{
            //Reload files
            let f1 = format!("snapshots/{}.bin", id);
            let f2 = format!("snapshots/{}.bin", id+1);
            println!("Loading files {} and {}", f1, f2);
            scene.snapshot1 = Snapshot::read(f1.as_str());
            scene.snapshot2 = Snapshot::read(f2.as_str());
            last_id = id;
        }
        let t_trig = (t/DURATION)*2.0*PI*45./360.;
        let (cam_x, cam_z) = t_trig.sin_cos();
        scene.camera_position.x = cam_x*10.0+center_x;
        scene.camera_position.z = cam_z*10.0;
        scene.transform  = (t/SNAPSHOT_DURATION).fract();
        save_render(renderer, &scene, format!("frames/image{}.png", frame_number).as_str());
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

fn render_pixel<T: Renderer<J>, J: RayResolver>(renderer: &T, scene: &SceneData, x: u32, y: u32) -> Vector3{
    let start = scene.camera_position;
    let _x: f32 = (x as f32/WIDTH_F-1f32)*ASPECT_RATIO;
    let _y: f32 = -(y as f32/HEIGHT_F-1f32);
    let ray_dir = scene.get_look_matrix().multiply(utilities::Vector3::new(_x, _y, FOV).normalized());
    return renderer.render(start, ray_dir, scene.clone(), WIDTH, HEIGHT);
}