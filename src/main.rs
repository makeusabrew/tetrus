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
    pub shapes: ShapeSet,
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

struct ShapeSet(Color, Vec<Vec<Vec<u8>>>);

impl ShapeSet {
    pub fn from_index(piece_type: usize) -> ShapeSet {
        match piece_type {
            0 => ShapeSet(
                Color::MAGENTA,
                T_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            1 => ShapeSet(
                Color::YELLOW,
                O_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            2 => ShapeSet(
                Color::GREEN,
                S_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            3 => ShapeSet(
                Color::RED,
                Z_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            4 => ShapeSet(
                COLOR_ORANGE,
                L_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            5 => ShapeSet(
                Color::BLUE,
                J_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
            _ => ShapeSet(
                Color::CYAN,
                I_PIECES.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec(),
            ),
        }
    }
    pub fn get(&self, angle: usize) -> Vec<Vec<u8>> {
        self.1[angle].clone()
    }
    pub fn len(&self) -> usize {
        self.1.len()
    }
    pub fn colour(&self) -> Color {
        self.0
    }
}

impl Piece {
    pub fn random() -> Piece {
        let index = rand::random::<usize>() % 7;
        let shapes = ShapeSet::from_index(index);
        Piece {
            shapes,
            angle: 0,
            column: 4,
            row: 0,
            tick_time: Instant::now(),
        }
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

    let mut piece = Piece::random();
    let mut score = Score {
        points: 0,
        lines: 0,
    };

    /*
     * @TODO:
     *
     * - collision detection with play field edges
     * - collision detection with other block edges
     * - don't detect collisions _below_ until we're about to tick again
     * - score display
     * - some shadowing on blocks to make them easier to distinguish
     * - space bar support
     * - input handling should detect press/release instead of just down events
     * - sound
     */

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

        let current_shape = piece.shapes.get(piece.angle);

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

                let current_index = x as usize + y;

                if current_index >= BLOCK_COUNT || blocks[current_index] != Color::BLACK {
                    // copy all the piece blocks into the background blocks array
                    for (row_index, piece_row) in current_shape.iter().enumerate() {
                        // the -1 here is very important. As soon as anything goes off screen or collides with something below it
                        // we need to snap it back up a row into place
                        let y = (piece.row + row_index - 1) * COLUMN_COUNT;
                        for (piece_index, block) in piece_row.iter().enumerate() {
                            let x = piece.column + piece_index as i32;
                            if *block == 0 || x < 0 {
                                continue;
                            }
                            blocks[x as usize + y] = piece.shapes.colour();
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
                    piece = Piece::random();
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

            canvas.set_draw_color(Color::GREY);
            canvas
                .fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w))
                .unwrap();
            let (start_offset, end_offset) = if colour.rgb() != (0, 0, 0) {
                (1, 2)
            } else {
                (0, 0)
            };
            canvas.set_draw_color(colour);
            canvas
                .fill_rect(Rect::new(
                    x as i32 + start_offset,
                    y as i32 + start_offset,
                    rect_w - end_offset,
                    rect_w - end_offset,
                ))
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
                canvas.set_draw_color(Color::GREY);
                canvas
                    .fill_rect(Rect::new(x as i32, y as i32, rect_w, rect_w))
                    .unwrap();

                canvas.set_draw_color(piece.shapes.colour());
                canvas
                    .fill_rect(Rect::new(
                        x as i32 + 1,
                        y as i32 + 1,
                        rect_w - 2,
                        rect_w - 2,
                    ))
                    .unwrap();
            }
        }
        canvas.present();
    }
}
