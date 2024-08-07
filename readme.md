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
This is a ray tracer with a GUI written from scratch, entirely in [Rust](https://www.rust-lang.org/).
For the GUI the [minifb](https://github.com/emoon/rust_minifb/) library for rust was used.

## System requirements
Unix based OS such as Linux or Mac OS

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [minifb](https://crates.io/crates/minifb)
- [png](https://crates.io/crates/png)
- [rand](https://crates.io/crates/rand)
- [threadpool](https://crates.io/crates/threadpool)


## Features
- Four shapes: `Cube`, `Sphere`, `Flat plane` and `Cylinder`.
- Four materials:  `Transparency`, `Reflective` and `Light`.
- Ability to change ambient brightness by changing the `PointLight` value.
- Rayon multithreading for faster rendering 🚀


### Camera Settings

To change the sample size, camera position, focal length, looking at and resolution, change the following in `main.rs`:
```rust
 let camera = Arc::new(Camera::new(
    origin,
    lookat,
    Vec3::new(0.0, 0.5, 3.5),
    WIDTH as f32 / HEIGHT as f32,
    90.0,
    0.05,
    2.0, 
));
```

### Brightness
```rust
 scene.add_light(Box::new(PointLight::new(
    Vec3::new(0.0, 1.5, -1.0),
    (1.0, 1.0, 1.0),
    0.9,
    15.0,
))); // Change the 0.01 to a value between 0.0 and 1.0. 1.0 being max, 0.0 being min.
```

### Objects

To create the objects, go to `main.rs` to initialize the objects, and add them to the `objects` vector using `Arc::new()`:
To create a objects, use the below, e.g. `Sphere` struct from `sphere.rs` in `scene.rs`. Here's an example:

```rust
//spere
 scene.add_object(Box::new(Sphere::new(
    Vec3::new(-1.5, 0.5, -1.0),
    0.5,
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    },
    0.9,
    0.0,
)));
//cube
scene.add_object(Box::new(Cube::new(
    Vec3::new(0.0, 0.75, -2.5),
    Vec3::new(0.75, 0.75, 0.75), // demi-taille du cube
    Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    },
    0.5,
    0.0,
)));
//cylinder
scene.add_object(Box::new(Cylinder::new(
    Vec3::new(1.5, 0.5, -1.0),
    0.5,
    1.5,
    Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    },
    0.2,
    0.0,
)));

```

### Textures
```rust
Transparency
Reflective
```

### Colors
There are a wide range of colors to choose from. These are just a small sample of all the available colors.
```rust
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}
```



### Figures
If you want to add new figure, just add it to the ***world***.

#### Example - sphere
```
Sphere::new(
        Vec3::new(-1.5, 0.5, -1.0),
        0.5,
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        0.9,
        0.0,
    );
```

### Camera
Camera has some interesting options as well.
```
Camera::new(
        origin,
        lookat,
        Vec3::new(0.0, 0.5, 3.5),
        aspect: WIDTH as f32 / HEIGHT as f32,
        fovy: 90.0,
        aperture: 0.05,
        focus_dist: 2.0,
    )
```
*origin* and *look_at* are describing from which point and to which direction camera is looking. Parameter *fovy* is setting camera view angle - this can be used as zoom in and out - for 90.0 it will be zoomed out an at 30.0-45.0 degrees it will be zoomed in to the objects. *aperture* is used for focus depth - the higher number, the less onjects will be in focus - this creates blur effect.

## Prerequisites
[dependencies]
minifb = "0.27"
png = "0.15.0"
rand = "0.3"
threadpool = "1.7"

This will create both ``raytracer.ppm`` and the window génerated instantly by minifb displaying your scene.
#### Authored by: [Mouhamadou Fadilou Diop](https://learn.zone01dakar.sn/git/mouhamadoufadiop/rt), [Daibou Ba](https://learn.zone01dakar.sn/git/daiba), [Ibrahima Diallo](https://learn.zone01dakar.sn/git/ediallo), [Mamadou Baldé](https://learn.zone01dakar.sn/git/mabalde)
###### Completed during [zone01-dakar](https://learn.zone01dakar.sn/) full-stack development course.
#### Project Description: [here](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)




