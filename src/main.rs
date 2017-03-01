extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::cmp::min;
use std::error;
use std::f64;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[macro_use] mod types;

mod consts;
mod object;
mod pixel;
mod renderer;
mod screen;
mod texture;
mod utils;

use consts::*;
use object::Object;
use renderer::Renderer;
use types::*;

macro_rules! main_try {
    ($x:expr) => {{
        let var = $x;
        if let Err(e) = var {
            error(&*e);
            return;
        }
        var.unwrap()
    }}
}


fn main() {
    // Initialize screen.
    let sdl_context = sdl2::init().unwrap();
    let screen = main_try!(screen::GraphicalScreen::new(
        "softraster",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &sdl_context,
    ));
    let mut renderer = Renderer::new(screen);
    let mut event_pump = sdl_context.event_pump().unwrap();


    // Set perspective transform.
    renderer.set_transform({
        let screen_scale = (min(SCREEN_WIDTH, SCREEN_HEIGHT) / 2) as f64;
        Transform::translate(pt_2d![
            (SCREEN_WIDTH  / 2) as Coord,
            (SCREEN_HEIGHT / 2) as Coord
        ])
        * Transform::scale(screen_scale, screen_scale, 1.)
        * Transform::perspective()
    });

    // Set up scene.
    let mut objects: Vec<Object> = main_try!(init_objects());
    renderer.set_light_pos(pt![10., 0., 10.]);

    // State variables.
    let mut paused = false;
    let mut step = true;
    let mut frame_dirty = true;
    let mut time = 0.;

    // Main loop.
    'main_loop: loop {
        // Time frame length.
        let frame_start = Instant::now();

        // Parse events.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                },

                Event::KeyDown { keycode: Some(code), .. } => {
                    match code {
                        Keycode::P      => paused = !paused,
                        Keycode::Space  => step = true,
                        _ => {}
                    }
                },

                _ => {},
            }
        }

        if step || !paused {
            step = false;

            // Update stuff.
            time += TIME_PER_TICK;
            objects[0].rotate_y(TIME_PER_TICK);
            objects[0].rotate_x(TIME_PER_TICK);
            frame_dirty = true;

            // Draw stuff.
            if frame_dirty {
                // Render image.
                renderer.clear();
                for object in &objects {
                    object.render(&mut renderer);
                }
                main_try!(renderer.display());

                frame_dirty = false;
            }
        }

        // Sleep until end-of-frame.
        let frame_duration = Instant::now() - frame_start;
        let fps = NANOS_PER_SECOND / frame_duration.subsec_nanos();
        if fps < TARGET_FPS { println!("slowed down!"); }
        let max_sleep = Duration::new(0, FRAME_LEN_NANOS);
        if max_sleep > frame_duration {
            thread::sleep(max_sleep - frame_duration);
        }
    }
}

fn error(err: &error::Error) {
    println!("error: {}", err);
    process::exit(-1);
}

fn init_objects() -> Result<Vec<Object>, Box<error::Error>> {
    let mut objects = Vec::new();

    objects.push({
        let size = 3.;
        try!(Object::from_resource_file("cube.obj"))
            .scaled(size, size, size)
            .translated(pt![0., 0., -10.])
            .rotated_x(f64::consts::PI / 4.)
    });

    Ok(objects)
}
