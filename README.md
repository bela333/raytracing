# Raytracer
A simple Path Tracer written in rust using Rayon.

## Running
```
$ git clone https://github.com/bela333/raytracing.git
$ cargo run --release
```

Most of the time it requires an `env.exr` file for environmental lighting.
[This is the one I use for the example renders.](https://hdrihaven.com/hdri/?h=christmas_photo_studio_02 "This is the one I use for the example renders")

Often times you would find that you get an .hdr file instead of an .exr one.
I used [this](https://convertio.co/hdr-exr/) converter to solve that issue. Any reasonable offline alternatives would be much appreciated.

## Current render
![Current render](/image.png)