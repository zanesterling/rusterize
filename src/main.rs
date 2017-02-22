extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::error;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod pixel;
mod renderer;
mod screen;
mod texture;
mod types;
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
    let r = 50f64;
    let mut theta = 0f64;
    let mut paused = false;

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

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                }

                _ => {},
            }
        }

        if !paused {
            // Update stuff.
            theta += 0.1;

            // Draw stuff.
            renderer.clear();
            renderer.translate((SCREEN_WIDTH / 2) as Coord, (SCREEN_HEIGHT / 2) as Coord);
            renderer.draw_line(
                (-r * theta.cos()) as i16, (-r * theta.sin()) as i16,
                ( r * theta.cos()) as i16, ( r * theta.sin()) as i16,
            );
            main_try!(renderer.display());
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
