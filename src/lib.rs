extern crate sdl2;

#[macro_use] pub mod types;
pub mod object;
pub mod pixel;
pub mod renderer;
pub mod screen;

use sdl2::event::Event as SdlEvent;

use std::error;
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod texture;
mod utils;

use renderer::Renderer;
use screen::GraphicalScreen;


const NANOS_PER_SECOND: u32 = 1_000_000_000;

type InitFunc<WorldState, S: screen::Screen> =
    fn (&mut Renderer<S>) -> Result<WorldState, Box<error::Error>>;
type ParseEventFunc         = fn (&mut LoopState, SdlEvent);
type UpdateFunc<WorldState> = fn (&mut LoopState, &mut WorldState) -> bool;
type RenderFunc<WorldState, S: screen::Screen> =
    fn (&mut Renderer<S>, &WorldState);

pub fn main_loop<'a, WorldState>
(
    screen_config: ScreenConfig,
    init: InitFunc<WorldState, screen::GraphicalScreen<'a>>,

    parse_event: ParseEventFunc,
    update:      UpdateFunc<WorldState>,
    render:      RenderFunc<WorldState, screen::GraphicalScreen<'a>>,
)
    -> Result<(), Box<error::Error>>
{
    // Initialize screen.
    let sdl_context = sdl2::init().unwrap();
    let screen = try!(GraphicalScreen::new(
        screen_config.title,
        screen_config.width,
        screen_config.height,
        &sdl_context,
    ));
    let mut renderer = Renderer::new(screen);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world_state = try!(init(&mut renderer));

    // Main loop.
    let frame_len_nanos = NANOS_PER_SECOND / screen_config.target_fps;
    let mut loop_state = LoopState::new();
    while loop_state.running {
        // Time frame length.
        let frame_start = Instant::now();

        // Update and render frame.
        for event in event_pump.poll_iter() {
            parse_event(&mut loop_state, event);
        }
        if loop_state.should_tick() {
            loop_state.step = false;
            let frame_dirty = update(&mut loop_state, &mut world_state);
            if  frame_dirty { render(&mut renderer, &world_state); }
        }

        // Sleep until end of frame.
        let frame_len = Instant::now() - frame_start;
        let target_frame_len = Duration::new(0, frame_len_nanos);
        if frame_len < target_frame_len {
            thread::sleep(target_frame_len - frame_len);
        } else {
            println!("slowed down!");
        }
    }

    Ok(())
}

pub struct ScreenConfig {
    pub title:  &'static str,
    pub width:  u32,
    pub height: u32,
    pub target_fps: u32
}

pub struct LoopState {
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

