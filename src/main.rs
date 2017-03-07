#[macro_use] extern crate rusterize;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rusterize::object::Object;
use rusterize::renderer::Renderer;
use rusterize::screen;
use rusterize::types::*;

use std::cmp::min;
use std::error;
use std::f64;
use std::path::Path;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod consts;
use consts::*;

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
    renderer.set_light_pos(pt![10., 0., 10.]);

    // State variables.
    let mut world_state = {
        let objects: Vec<Object> = main_try!(init_objects());
        WorldState {
            time: 0.,
            objects: objects,
        }
    };

    // Main loop.
    let mut loop_state = LoopState::new();
    while loop_state.running {
        // Time frame length.
        let frame_start = Instant::now();

        {
            // Update and render frame.
            for event in event_pump.poll_iter() { parse_event(&mut loop_state, event); }
            if loop_state.should_tick() {
                loop_state.step = false;
                let frame_dirty = update(&mut loop_state, &mut world_state);
                if  frame_dirty { render(&mut renderer, &world_state); }
            }
        }

        // Sleep until end of frame.
        let frame_len = Instant::now() - frame_start;
        let target_frame_len = Duration::new(0, FRAME_LEN_NANOS);
        if frame_len < target_frame_len {
            thread::sleep(target_frame_len - frame_len);
        } else {
            println!("slowed down!");
        }
    }
}

struct LoopState {
    pub running: bool,
    pub paused: bool,
    pub step: bool,
}

impl LoopState {
    pub fn new() -> LoopState {
        LoopState {
            running: true,
            paused: false,
            step:   false,
        }
    }

    pub fn should_tick(&self) -> bool {
        self.paused || self.step
    }
}


struct WorldState {
    time: f64,
    objects: Vec<Object>
}


fn parse_event(loop_state: &mut LoopState, event: Event) {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            loop_state.running = false;
        },

        Event::KeyDown { keycode: Some(code), .. } => {
            match code {
                Keycode::P      => loop_state.paused = !loop_state.paused,
                Keycode::Space  => loop_state.step   = true,
                _ => {}
            }
        },

        _ => {},
    }
}

// Returns true if the frame is made dirty, else false.
fn update(_: &mut LoopState, world_state: &mut WorldState) -> bool {
    // Update stuff.
    world_state.time += TIME_PER_TICK;
    world_state.objects[0].rotate_y(TIME_PER_TICK);
    world_state.objects[0].rotate_x(TIME_PER_TICK);
    true // frame dirty
}

fn render<T: rusterize::screen::Screen>(
    renderer: &mut Renderer<T>,
    world_state: &WorldState
) {
    // Render image.
    renderer.clear();
    for object in &world_state.objects {
        object.render(renderer);
    }
    main_try!(renderer.display());
}

fn init_objects() -> Result<Vec<Object>, Box<error::Error>> {
    let mut objects = Vec::new();

    objects.push({
        let size = 3.;
        load_object_from_file("cube.obj")?
            .scaled(size, size, size)
            .translated(pt![0., 0., -20.])
            .rotated_x(f64::consts::PI / 4.)
    });

    Ok(objects)
}


fn error(err: &error::Error) {
    println!("error: {}", err);
    process::exit(-1);
}

fn load_object_from_file(filename: &str)
    -> Result<Object, Box<error::Error>>
{
    Object::from_file(&Path::new(RES_DIR_PATH).join(filename))
}
