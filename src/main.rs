extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BLOCK_SIZE: u32 = 30;

const PLAYFIELD_START_X: u32 = 250;
const PLAYFIELD_START_Y: u32 = 0;

fn main() {
    // 10x20 area
    let mut blocks = [0u8; 200];

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("tetrus", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // render blocks
        for (i, block) in blocks.iter().enumerate() {
            let block = *block;
            let i = i as u32;
            let colour = block + i as u8;
            let x = PLAYFIELD_START_X + ((i % 10) * BLOCK_SIZE);
            let y = PLAYFIELD_START_Y + ((i / 10) * BLOCK_SIZE);
            canvas.set_draw_color(Color::RGB(colour, colour, colour));
            canvas.fill_rect(Rect::new(x as i32, y as i32, BLOCK_SIZE, BLOCK_SIZE)).unwrap();
        }

        canvas.present();
    }
}
