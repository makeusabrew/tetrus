extern crate sdl2;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const BLOCK_SIZE: usize = 30;
const PLAYFIELD_START_X: usize = 250;
const PLAYFIELD_START_Y: usize = 0;

const COLUMN_COUNT: usize = 10;
const ROW_COUNT: usize = 20;
const BLOCK_COUNT: usize = COLUMN_COUNT * ROW_COUNT;

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
const COLOR_ORANGE: Color = Color::RGBA(255, 165, 0, 255);

struct Piece {
    pub shapes: Vec<Vec<Vec<u8>>>,
    pub colour: Color,
    pub angle: usize,
    pub column: i32,
    pub row: usize,
    pub tick_time: Instant,
}

struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

struct Score {
    pub points: u32,
    pub lines: u32,
}

fn map_shapes(piece_type: usize) -> (Color, Vec<Vec<Vec<u8>>>) {
    match piece_type {
        0 => (
            Color::MAGENTA,
            T_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        1 => (
            Color::YELLOW,
            O_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        2 => (
            Color::GREEN,
            S_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        3 => (
            Color::RED,
            Z_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        4 => (
            COLOR_ORANGE,
            L_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        5 => (
            Color::BLUE,
            J_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
        _ => (
            Color::CYAN,
            I_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
        ),
    }
}

fn get_random_piece() -> Piece {
    let index = rand::random::<usize>() % 7;
    let (colour, shapes) = map_shapes(index);
    Piece {
        shapes,
        colour,
        angle: 0,
        column: 4,
        row: 0,
        tick_time: Instant::now(),
    }
}

fn main() {
    // 10x20 area
    let mut blocks = [Color::BLACK; BLOCK_COUNT];

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("tetrus", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut piece = get_random_piece();
    let mut score = Score {
        points: 0,
        lines: 0,
    };

    'running: loop {
        /*
         * Input handling
         */
        let mut input = Input {
            left: false,
            right: false,
            up: false,
            down: false,
        };
        for event in event_pump.poll_iter() {
            // @TODO: don't directly mutate x/y here, signal that we want to move instead
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    input.left = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    input.right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    input.down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    input.up = true;
                }
                _ => {}
            }
        }

        if piece.tick_time.elapsed().as_secs() >= 1 {
            piece.tick_time = Instant::now();
            if !input.down {
                piece.row += 1;
            }
        }
        if input.up {
            piece.angle += 1;
            if piece.angle == piece.shapes.len() {
                piece.angle = 0;
            }
        }
        if input.down {
            piece.row += 1;
        }
        if input.left {
            piece.column -= 1;
        } else if input.right {
            piece.column += 1;
        }

        let current_shape = piece.shapes[piece.angle as usize].clone();

        /*
         * Collision detection
         */
        'collision: for (row_index, piece_row) in current_shape.iter().enumerate() {
            let y = (piece.row + row_index) * COLUMN_COUNT;
            for (col_index, block) in piece_row.iter().enumerate() {
                let x = piece.column + col_index as i32;
                if *block == 0 || x < 0 {
                    continue;
                }

                let row_below = (x as usize + y) + COLUMN_COUNT;

                // we're either on the last row or there's something underneath us - stop here
                if row_below >= BLOCK_COUNT || blocks[row_below] != Color::BLACK {
                    // copy all the piece blocks into the background blocks array
                    for (row_index, piece_row) in current_shape.iter().enumerate() {
                        let y = (piece.row + row_index) * COLUMN_COUNT;
                        for (piece_index, block) in piece_row.iter().enumerate() {
                            let x = piece.column + piece_index as i32;
                            if *block == 0 || x < 0 {
                                continue;
                            }
                            blocks[x as usize + y] = piece.colour;
                        }
                    }

                    let mut lines = vec![];

                    // get a count of how many lines we've cleared plus their indices
                    for row in 0..ROW_COUNT {
                        for col in 0..COLUMN_COUNT {
                            if blocks[(row * COLUMN_COUNT) + col] == Color::BLACK {
                                break;
                            }
                            if col == COLUMN_COUNT - 1 {
                                lines.push(row);
                            }
                        }
                    }

                    // shift all rows above each line down by one
                    for line in &lines {
                        for row in (1..=*line).rev() {
                            for col in 0..COLUMN_COUNT {
                                blocks[(row * COLUMN_COUNT) + col] =
                                    blocks[((row - 1) * COLUMN_COUNT) + col];
                            }
                        }
                    }

                    let num_lines = lines.len() as u32;
                    score.lines += num_lines;
                    score.points += (num_lines * num_lines) * 100;

                    // as soon as we hit anything, spawn a new piece and don't look for any further collisions
                    piece = get_random_piece();
                    break 'collision;
                }
            }
        }

        /*
         * Rendering
         */
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // existing blocks first
        for (i, block) in blocks.iter().enumerate() {
            let colour = *block;
            let x = PLAYFIELD_START_X + ((i % 10) * BLOCK_SIZE);
            let y = PLAYFIELD_START_Y + ((i / 10) * BLOCK_SIZE);
            let rect_w = BLOCK_SIZE as u32;
            canvas.set_draw_color(colour);
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
                let x = PLAYFIELD_START_X as i32
                    + ((piece.column + piece_index as i32) * BLOCK_SIZE as i32);
                let rect_w = BLOCK_SIZE as u32;
                canvas.set_draw_color(piece.colour);
                canvas
                    .fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w))
                    .unwrap();
            }
        }
        canvas.present();
    }
}
