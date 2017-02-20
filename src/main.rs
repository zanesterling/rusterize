mod screen;

const SCREEN_WIDTH:  usize = 20;
const SCREEN_HEIGHT: usize = 20;

fn main() {
    let screen = screen::Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    screen.render();
}
