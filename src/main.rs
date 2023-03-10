use sdl2::event::Event;

pub mod renderer;
use renderer::Renderer;

pub mod math;

pub mod input;
pub use input::*;

mod game;
use game::*;

mod collision;

mod sprite_sheet;

mod resources;
use resources::*;

fn main() {
    let sdl_context = sdl2::init().expect("SDL: Failed to init SDL");

    // Init video
    let video = sdl_context
        .video()
        .expect("SDL: Failed to init Video subsystem");
    let window = video
        .window("Minesweeper", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");
    let mut renderer = Renderer::new(&window, &video).unwrap();

    // Init input
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut resources = Resources::new();

    // Init game
    let mut game_state= GameState::new();

    let mut previous_frame = std::time::Instant::now();

    'running: loop {
        game_state.delta_time = previous_frame.elapsed().as_secs_f32();
        previous_frame = std::time::Instant::now();
        resources.check_for_changes();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => { 
                    game_state.event(event)
                }
            }
        }

        let input = Input::from_pump(&event_pump);

        game_state.update(&input);
        game_state.draw(&mut renderer, &resources);
        renderer.swap(&window);
    }
}

