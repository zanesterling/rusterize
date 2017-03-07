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
    main_try!(rusterize::main_loop(
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
    ));
}

struct WorldState {
    time: f64,
    objects: Vec<Object>
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

fn load_object_from_file(filename: &str)
    -> Result<Object, Box<error::Error>>
{
    Object::from_file(&Path::new(RES_DIR_PATH).join(filename))
}

fn error(err: &error::Error) {
    println!("error: {}", err);
    process::exit(-1);
}
