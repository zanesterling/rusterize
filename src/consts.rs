pub const SCREEN_WIDTH:  u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub const TARGET_FPS:       u32 = 60;
pub const NANOS_PER_SECOND: u32 = 1_000_000_000;
pub const FRAME_LEN_NANOS:  u32 = NANOS_PER_SECOND / TARGET_FPS;
pub const TIME_PER_TICK:    f64 = 1. / (TARGET_FPS as f64);

pub const RES_DIR_PATH: &'static str = "res";
