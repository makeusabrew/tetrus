extern crate sdl2;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BLOCK_SIZE: usize = 30;
const BLOCK_PER_ROW: usize = 10;
const BLOCK_COUNT: usize = 200;

const PLAYFIELD_START_X: usize = 250;
const PLAYFIELD_START_Y: usize = 0;

const T_PIECE_N: [[u8; 3]; 3] = [
    [0, 200, 0],
    [200, 200, 200],
    [0, 0, 0],
];
const T_PIECE_E: [[u8; 3]; 3] = [
    [0, 200, 0],
    [0, 200, 200],
    [0, 200, 0],
];
const T_PIECE_S: [[u8; 3]; 3] = [
    [0, 0, 0],
    [200, 200, 200],
    [0, 200, 0],
];
const T_PIECE_W: [[u8; 3]; 3] = [
    [0, 200, 0],
    [200, 200, 0],
    [0, 200, 0],
];

const T_PIECES: [[[u8; 3]; 3]; 4] = [T_PIECE_N, T_PIECE_E, T_PIECE_S, T_PIECE_W];


fn main() {
    // 10x20 area
    let mut blocks = [0u8; BLOCK_COUNT];

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("tetrus", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut current_piece_col: usize = 0;
    let mut current_piece_row = 0;
    //§let mut current_piece = [[0u8; 3]; 3];
    let mut current_piece_index = 0;

    let mut tick_time = Instant::now();

    'running: loop {
        // @TODO: don't directly mutate x/y here, signal that we want to move
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    current_piece_col -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    current_piece_col += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    current_piece_row += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    current_piece_index += 1;
                    if current_piece_index == 4 {
                        current_piece_index = 0;
                    }
                }
                _ => {}
            }
        }

        let current_piece = T_PIECES[current_piece_index];

        if tick_time.elapsed().as_secs() >= 1 {
            tick_time = Instant::now();
            current_piece_row += 1;
        }

        'collision: for (row_index, piece_row) in current_piece.iter().enumerate() {
            let y = (current_piece_row + row_index) * BLOCK_PER_ROW;
            for (col_index, piece_val) in piece_row.iter().enumerate() {
                let colour = *piece_val;
                if colour == 0 {
                    continue
                }
                let x = current_piece_col + col_index;
                let idx = (x + y) + BLOCK_PER_ROW;
                if idx >= BLOCK_COUNT  || blocks[idx] != 0 {
                    // write all the piece blocks into the background blocks array
                    for (row_index, piece_row) in current_piece.iter().enumerate() {
                        let row_offset = (row_index + current_piece_row) * 10;
                        let col_offset = current_piece_col as usize;
                        for (piece_index, piece_block) in piece_row.iter().enumerate() {
                            if *piece_block == 0 {
                                continue;
                            }
                            blocks[row_offset + piece_index + col_offset] = *piece_block;
                        }
                    }
                    current_piece_row = 0;
                    current_piece_col = 0;
                    break 'collision;
                }
            }
        }

        /*
         * Render
         */
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // fixed blocks first
        for (i, block) in blocks.iter().enumerate() {
            let block = *block;
            let colour = block;
            let x = PLAYFIELD_START_X + ((i % 10) * BLOCK_SIZE);
            let y = PLAYFIELD_START_Y + ((i / 10) * BLOCK_SIZE);
            let rect_w = BLOCK_SIZE as u32;
            canvas.set_draw_color(Color::RGB(colour, colour, colour));
            canvas.fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w)).unwrap();
        }

        // current piece
        for (row_index, piece_row) in current_piece.iter().enumerate() {
            let y = PLAYFIELD_START_Y + ((current_piece_row + row_index) * BLOCK_SIZE);

            for (piece_index, piece_val) in piece_row.iter().enumerate() {
                let colour = *piece_val;
                if colour == 0 {
                    continue
                }
                let x = PLAYFIELD_START_X + ((current_piece_col + piece_index) * BLOCK_SIZE);
                let rect_w = BLOCK_SIZE as u32;
                canvas.set_draw_color(Color::RGB(colour, colour, colour));
                canvas.fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w)).unwrap();
            }
        }
        canvas.present();
    }
}
