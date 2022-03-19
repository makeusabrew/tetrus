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

const T_PIECES: [[[u8; 3]; 3]; 4] = [
    [[0, 1, 0], [1, 1, 1], [0, 0, 0]],
    [[0, 1, 0], [0, 1, 1], [0, 1, 0]],
    [[0, 0, 0], [1, 1, 1], [0, 1, 0]],
    [[0, 1, 0], [1, 1, 0], [0, 1, 0]],
];
const O_PIECES: [[[u8; 2]; 2]; 1] = [[[1, 1], [1, 1]]];
const S_PIECES: [[[u8; 3]; 3]; 4] = [
    [[0, 1, 1], [1, 1, 0], [0, 0, 0]],
    [[0, 1, 0], [0, 1, 1], [0, 0, 1]],
    [[0, 0, 0], [0, 1, 1], [1, 1, 0]],
    [[1, 0, 0], [1, 1, 0], [0, 1, 0]],
];
const Z_PIECES: [[[u8; 3]; 3]; 4] = [
    [[1, 1, 0], [0, 1, 1], [0, 0, 0]],
    [[0, 0, 1], [0, 1, 1], [0, 1, 0]],
    [[0, 0, 0], [1, 1, 0], [0, 1, 1]],
    [[0, 1, 0], [1, 1, 0], [1, 0, 0]],
];
const L_PIECES: [[[u8; 3]; 3]; 4] = [
    [[0, 0, 1], [1, 1, 1], [0, 0, 0]],
    [[0, 1, 0], [0, 1, 0], [0, 1, 1]],
    [[0, 0, 0], [1, 1, 1], [1, 0, 0]],
    [[1, 1, 0], [0, 1, 0], [0, 1, 0]],
];
const J_PIECES: [[[u8; 3]; 3]; 4] = [
    [[1, 0, 0], [1, 1, 1], [0, 0, 0]],
    [[0, 1, 1], [0, 1, 0], [0, 1, 0]],
    [[0, 0, 0], [1, 1, 1], [0, 0, 1]],
    [[0, 1, 0], [0, 1, 0], [1, 1, 0]],
];
const I_PIECES: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
];

struct Piece {
    pub shapes: Vec<Vec<Vec<u8>>>,
    pub angle: usize,
    pub column: usize,
    pub row: usize,
}

fn map_shapes(piece_type: usize) -> Vec<Vec<Vec<u8>>> {
    match piece_type {
        0 => T_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        1 => O_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        2 => S_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        3 => Z_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        4 => L_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        5 => J_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        _ => I_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
    }
}

fn get_random_piece() -> Piece {
    let index = rand::random::<usize>() % 7;
    Piece {
        shapes: map_shapes(index),
        angle: 0,
        column: 0,
        row: 0,
    }
}

fn main() {
    // 10x20 area
    let mut blocks = [0u8; BLOCK_COUNT];

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("tetrus", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut tick_time = Instant::now();

    let mut piece = get_random_piece();

    'running: loop {
        // @TODO: don't directly mutate x/y here, signal that we want to move
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    piece.column -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    piece.column += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    piece.row += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    piece.angle += 1;
                    if piece.angle == piece.shapes.len() {
                        piece.angle = 0;
                    }
                }
                _ => {}
            }
        }

        if tick_time.elapsed().as_secs() >= 1 {
            tick_time = Instant::now();
            piece.row += 1;
        }

        let current_shape = piece.shapes[piece.angle as usize].clone();

        'collision: for (row_index, piece_row) in current_shape.iter().enumerate() {
            let y = (piece.row + row_index) * BLOCK_PER_ROW;
            for (col_index, piece_val) in piece_row.iter().enumerate() {
                let colour = *piece_val;
                if colour == 0 {
                    continue;
                }
                let x = piece.column + col_index;
                let idx = (x + y) + BLOCK_PER_ROW;
                if idx >= BLOCK_COUNT || blocks[idx] != 0 {
                    // write all the piece blocks into the background blocks array
                    for (row_index, piece_row) in current_shape.iter().enumerate() {
                        let row_offset = (row_index + piece.row) * 10;
                        let col_offset = piece.column as usize;
                        for (piece_index, piece_block) in piece_row.iter().enumerate() {
                            if *piece_block == 0 {
                                continue;
                            }
                            blocks[row_offset + piece_index + col_offset] = *piece_block * 100;
                        }
                    }
                    piece = get_random_piece();
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
            canvas
                .fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w))
                .unwrap();
        }

        // current piece
        for (row_index, piece_row) in current_shape.iter().enumerate() {
            let y = PLAYFIELD_START_Y + ((piece.row + row_index) * BLOCK_SIZE);

            for (piece_index, piece_val) in piece_row.iter().enumerate() {
                let colour = *piece_val;
                if colour == 0 {
                    continue;
                }
                let x = PLAYFIELD_START_X + ((piece.column + piece_index) * BLOCK_SIZE);
                let rect_w = BLOCK_SIZE as u32;
                canvas.set_draw_color(Color::RGB(colour * 100, colour, colour));
                canvas
                    .fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w))
                    .unwrap();
            }
        }
        canvas.present();
    }
}
