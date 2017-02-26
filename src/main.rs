extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::error;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[macro_use] mod types;

mod pixel;
mod renderer;
mod screen;
mod texture;
mod utils;

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


const SCREEN_WIDTH:  u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const TARGET_FPS:       u32 = 60;
const NANOS_PER_SECOND: u32 = 1_000_000_000;
const FRAME_LEN_NANOS:  u32 = NANOS_PER_SECOND / TARGET_FPS;

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

    // State variables.
    let mut paused = false;
    let mut step = false;
    let mut frame_dirty = true;
    let mut theta = 0.0;

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

                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    paused = !paused;
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    step = true;
                }

                _ => {},
            }
        }

        if !paused || step {
            step = false;

            // Update stuff.
            theta += 0.01;
            frame_dirty = true;

            // Draw stuff.
            if frame_dirty {
                // Set transformation.
                renderer.clear_transform();
                renderer.rotate_y(theta);
                renderer.translate(pt![0., 0., -2.]);
                renderer.perspective();
                renderer.scale(200., 200., 1.);
                renderer.translate(pt_2d![
                    (SCREEN_WIDTH / 2)  as Coord,
                    (SCREEN_HEIGHT / 2) as Coord
                ]);

                // Render image.
                renderer.clear();
                renderer.fill_triangle([
                    pt_2d![-1., -1.],
                    pt_2d![-1.,  1.],
                    pt_2d![ 1., -1.],
                ]);
                renderer.fill_triangle([
                    pt_2d![ 1.,  1.],
                    pt_2d![-0.9,  1.],
                    pt_2d![ 1., -0.9],
                ]);
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
