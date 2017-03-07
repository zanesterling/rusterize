#[macro_use] extern crate rusterize;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rusterize::object::Object;
use rusterize::renderer::Renderer;
use rusterize::screen::Screen;
use rusterize::types::*;

use std::cmp::min;
use std::error;
use std::f64;
use std::path::Path;
use std::process;


pub const SCREEN_WIDTH:  u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const TARGET_FPS:    u32 = 60;
pub const TIME_PER_TICK: f64 = 1. / (TARGET_FPS as f64);

struct WorldState {
    time: f64,
    objects: Vec<Object>
}


fn main() {
    let result = rusterize::main_loop(
        rusterize::ScreenConfig {
            title:      "rusterize",
            width:      SCREEN_WIDTH,
            height:     SCREEN_HEIGHT,
            target_fps: TARGET_FPS,
        },
        init,
        parse_event,
        update,
        render
    );

    if let Err(e) = result {
        println!("error: {}", e);
        process::exit(-1);
    }
}

fn init<S: Screen>(renderer: &mut Renderer<S>)
    -> Result<WorldState, Box<error::Error>>
{
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
    let objects: Vec<Object> = try!(init_objects());
    Ok(
        WorldState {
            time: 0.,
            objects: objects,
        }
    )
}

fn parse_event(loop_state: &mut rusterize::LoopState, event: Event) {
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
fn update(_: &mut rusterize::LoopState, world_state: &mut WorldState) -> bool {
    // Update stuff.
    world_state.time += TIME_PER_TICK;
    world_state.objects[0].rotate_y(TIME_PER_TICK);
    world_state.objects[0].rotate_x(TIME_PER_TICK);
    true // frame dirty
}

fn render<T: Screen>(
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
        load_object_from_file("res/cube.obj")?
            .scaled(size, size, size)
            .translated(pt![0., 0., -20.])
            .rotated_x(f64::consts::PI / 4.)
    });

    Ok(objects)
}
