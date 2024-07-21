# rt
[Documentation](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
## Table of Contents
- [About](#about)
- [System requirements](#system-requirements)
- [Dependencies](#dependencies)
- [Installation/Running Instructions](#installationrunning-instructions)
- [Features](#features)
- [Adjusting objects and scene](#adjusting-objects-and-scene)
    - [Main settings](#main-settings)
    - [Materials](#materials)
    - [Figures](#figures)
    - [Camera](#camera)
   

## About
This is a Monte Carlo based ray tracer with a GUI written from scratch, entirely in [Rust](https://www.rust-lang.org/).
For the GUI the [GTK3](https://docs.gtk.org/gtk3/) library for rust was used.

## System requirements
Unix based OS such as Linux or Mac OS

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [GTK3](https://docs.gtk.org/gtk3/)


## Features
- Four shapes: `Cube`, `Sphere`, `Flat plane` and `Cylinder`.
- Four materials: `Diffusive`, `Glossy`, `Reflective` and `Light`.
- Ability to change ambient brightness by changing the `brightness` value.
- Rayon multithreading for faster rendering 🚀

## Run without GUI

To run program without launching the GUI:
```cargo run --release no-gui```

### Camera Settings

To change the sample size, camera position, focal length, looking at and resolution, change the following in `main.rs`:
```rust
let mut camera = CameraBuilder::new()
                    .sample_size(20)
                    .position_by_coordinates(Point::new(-6.0, 4.0, 15.0))
                    .focal_length(1.0)
                    .look_at(Point::new(0.0, 0.0, 0.0))
                    .resolution(1920, 1080)
                    .build();
```

### Brightness
```rust
 let scene = Arc::new(Scene::init(0.01)); // Change the 0.01 to a value between 0.0 and 1.0. 1.0 being max, 0.0 being min.
```

### Objects

To create the objects, go to `scene.rs` to initialize the objects, and add them to the `objects` vector using `Arc::new()`:
To create a objects, use the below, e.g. `Sphere` struct from `sphere.rs` in `scene.rs`. Here's an example:

```rust
let sphere = Sphere::new(position, radius, texture);
let cube = Cube::new(position, side_length, texture);
let plane = FlatPlane::new(position, radius, texture);
let cylinder = Cylinder::new(position, radius, height, texture);
```

### Textures
```rust
Diffusive(color)
Light(color)
Reflective
```

### Colors
There are a wide range of colors to choose from. These are just a small sample of all the available colors.
```rust
RGB::new() // Custom color in 255,255,255 format
RGB::random()
RGB::red()
RGB::green()
RGB::blue()
```

### Finalize the scene
```rust
// Initialize an object
let sphere = Sphere::new(
    Point::new(0.0, 1.0, 0.0),
    1.0, 
    Textures::Diffusive(RGB::red())
);

// more objects here...

// Add the objects to the scene in this vector
let objects: Objects = vec![
    Arc::new(sphere),
    Arc::(object2),
    // More objects here...
];

// Return the scene
Scene {
    objects, brightness
}
```

## Adjusting objects and scene

You can modify the `config.ron` file to add/remove/edit objects and alter technical details, such as lighting and camera properties.  

### Main settings
*light* - the value should be between 0-100, lower value will create darker picture

*samples* - is the amount of pixel samples for antialiasing, should be positive number. The higher value - the better picture quality you will get (but also will take more time to render the picture)

*width* and *height* - resolution of output image 

### Materials
There are three kinds of materials available: [Lambertian](https://en.wikipedia.org/wiki/Lambertian_reflectance), Metal, and Dielectric. Their formats are as follows:  
```
material: Lambertian(
    albedo: Vec3(0.4, 0.4, 1.0),   // R, G, B; 0.0-1.0
)
```
```
material: Metal(
    albedo: Vec3(0.4, 0.4, 1.0),   // R, G, B; 0.0-1.0
)
```
```
material: Dielectric(
    ref_idx: 0.5,                  // Refractive index; see https://en.wikipedia.org/wiki/Refractive_index
)
```

### Figures
If you want to add new figure, just add it to the ***world***.

#### Example - sphere
```
{
    "Sphere": (
    center: Vec3(2.0, 0.0, -1.0),
    radius: 0.5,
    material: Lambertian( albedo: Vec3(0.2, 0.2, 1.0) ) 
    )
},
```
For other figures, please refer to [config.ron](config.ron)

### Camera
Camera has some interesting options as well.
```
camera: (
        look_from: Vec3(1.0, 3.0, 4.0),
        look_at: Vec3(0.0, 0.0, -1.0),
        vup: Vec3(0.0, 1.0, 0.0),
        vfov: 45.0,
        aperture: 0.1,
    )
```
*look_from* and *look_at* are describing from which point and to which direction camera is looking. Parameter *vfov* is setting camra view angle - this can be used as zoom in and out - for 90.0 it will be zoomed out an at 30.0-45.0 degrees it will be zoomed in to the objects. *aperture* is used for focus depth - the higher number, the less onjects will be in focus - this creates blur effect.

## Prerequisites

Rust nightly and [FFmpeg](https://www.ffmpeg.org/) are required.  
Rust nightly is installed like so:  
``
rustup install nightly
``  
or  
``
rustup default nightly
``

## Running the program and rendering

``
make run
``

This will create both ``example.ppm`` and ``example.png`` displaying your scene.
#### Authored by: [Mouhamadou Fadilou Diop](https://learn.zone01dakar.sn/git/mouhamadoufadiop/rt), [Daibou Ba](https://learn.zone01dakar.sn/git/daiba), [Ibrahima Diallo](https://learn.zone01dakar.sn/git/ediallo), [Mamadou Baldé](https://learn.zone01dakar.sn/git/mabalde),[Alimoudine IDRISSOU ](https://learn.zone01dakar.sn/git/ialimoud) and [Ndiaga Ba](https://learn.zone01dakar.sn/git/nihiba)
###### Completed during [grit:lab](https://gritlab.ax/) full-stack development course.
#### Project Description: [here](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)

![closeup](https://github.com/bomanviktor/rt/assets/72476579/115a7a5e-e942-46e9-a75f-2246820571cf)
