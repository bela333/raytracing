extern crate image;
extern crate rand;
extern crate rand_distr;

pub mod ray_resolver;
pub mod ray_marcher;
pub mod renderer;
pub mod basic_renderer;
pub mod utilities;

use crate::renderer::Renderer;
use image::{ImageBuffer};

const WIDTH:u32 = 640;
const HEIGHT:u32 = 480;

const WIDTH_F:f32 = WIDTH as f32 / 2f32;
const HEIGHT_F:f32 = HEIGHT as f32 / 2f32;

const ASPECT_RATIO:f32 = WIDTH_F / HEIGHT_F;

fn main() {
    let marcher = ray_marcher::RayMarcher{
        max_distance: 100f32,
        max_steps: 500,
        epsilon: 0.0002f32
    };
    let renderer = basic_renderer::BasicRenderer{
        resolver: marcher
    };
    let scene = utilities::SceneData{

    };
    let image = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y|{
        let start = utilities::Vector3::new(0f32, 0f32, 0f32);
        let _x: f32 = (x as f32/WIDTH_F-1f32)*ASPECT_RATIO;
        let _y: f32 = -(y as f32/HEIGHT_F-1f32);
        let dir = utilities::Vector3::new(_x, _y, 1f32).normalized();
        let color = renderer.render(start, dir, scene.clone());
        image::Rgb(color.to_color_array())
    });
    image.save("image.png").unwrap();
    

}