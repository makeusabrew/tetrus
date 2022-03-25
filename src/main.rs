extern crate sdl2;

use std::thread;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
}

struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

#[derive(Debug)]
struct Score {
    pub points: u32,
    pub lines: u32,
}

macro_rules! shape_vec {
    ($a:expr) => {
        $a.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec()
    };
}

struct ShapeSet(Color, Vec<Vec<Vec<u8>>>, [u8; 2]);

impl ShapeSet {
    pub fn from_index(piece_type: usize) -> ShapeSet {
        match piece_type {
            0 => ShapeSet(Color::MAGENTA, shape_vec!(T_PIECES), [3, 2]),
            1 => ShapeSet(Color::YELLOW, shape_vec!(O_PIECES), [2, 2]),
            2 => ShapeSet(Color::GREEN, shape_vec!(S_PIECES), [3, 2]),
            3 => ShapeSet(Color::RED, shape_vec!(Z_PIECES), [3, 2]),
            4 => ShapeSet(COLOR_ORANGE, shape_vec!(L_PIECES), [3, 2]),
            5 => ShapeSet(Color::BLUE, shape_vec!(J_PIECES), [3, 2]),
            _ => ShapeSet(Color::CYAN, shape_vec!(I_PIECES), [4, 1]),
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
    pub fn width(&self, angle: usize) -> u8 {
        self.2[angle % 2]
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
        }
    }

    pub fn current_shape(&self) -> Vec<Vec<u8>> {
        self.shapes.get(self.angle)
    }

    pub fn width(&self) -> u8 {
        self.shapes.width(self.angle)
    }

    pub fn rotate(&mut self) {
        self.angle += 1;
        if self.angle == self.shapes.len() {
            self.angle = 0;
        }
    }
}

fn render_piece(canvas: &mut Canvas<Window>, piece: &Piece, sx: usize, sy: usize) {
    for (row_index, piece_row) in piece.current_shape().iter().enumerate() {
        let y = (sy + ((piece.row + row_index) * BLOCK_SIZE)) as i32;

        for (piece_index, piece_val) in piece_row.iter().enumerate() {
            let colour = *piece_val;
            if colour == 0 {
                continue;
            }
            let x = sx as i32 + ((piece.column + piece_index as i32) * BLOCK_SIZE as i32);
            let size = BLOCK_SIZE as u32;
            canvas.set_draw_color(Color::GREY);
            canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();

            canvas.set_draw_color(piece.shapes.colour());
            canvas
                .fill_rect(Rect::new(x + 1, y + 1, size - 2, size - 2))
                .unwrap();
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
    let mut next_piece = Piece::random();
    let mut score = Score {
        points: 0,
        lines: 0,
    };

    /*
     * @TODO:
     *
     * - score display
     * - space bar support
     * - sound
     */

    let mut last_rotation = Instant::now();
    let mut tick_time = Instant::now();
    'running: loop {
        let frame_start = Instant::now();
        /*
         * Input handling
         */
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let mut input = Input {
            left: false,
            right: false,
            up: false,
            down: false,
        };
        event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|key| match key {
                Keycode::Up => input.up = true,
                Keycode::Down => input.down = true,
                Keycode::Left => input.left = true,
                Keycode::Right => input.right = true,
                _ => {}
            });

        if tick_time.elapsed().as_secs() >= 1 {
            tick_time = Instant::now();
            piece.row += 1;
        } else if input.down {
            piece.row += 1;
        }

        if input.up && last_rotation.elapsed().as_millis() >= 100 {
            last_rotation = Instant::now();
            piece.rotate();

            let max = (COLUMN_COUNT - piece.width() as usize) as i32;
            piece.column = piece.column.min(max).max(0);
        }

        /*
         * Horizontal collision detection
         *
         * This can unset any left/right movement intended by the player
         */
        for (row_index, piece_row) in piece.current_shape().iter().enumerate() {
            let y = (piece.row + row_index) * COLUMN_COUNT;
            for (col_index, block) in piece_row.iter().enumerate() {
                if *block == 0 {
                    continue;
                }

                let x = piece.column + col_index as i32;

                // playfield left/right
                if x <= 0 {
                    input.left = false;
                } else if x >= COLUMN_COUNT as i32 - 1 {
                    input.right = false;
                } else {
                    // neighbouring blocks
                    // bit of a bodge - only assigned here because without first testing x we might overflow
                    let left_neighbour = x as usize + y - 1;
                    let right_neighbour = x as usize + y + 1;
                    if left_neighbour >= BLOCK_COUNT || blocks[left_neighbour] != Color::BLACK {
                        input.left = false;
                    }
                    if right_neighbour >= BLOCK_COUNT || blocks[right_neighbour] != Color::BLACK {
                        input.right = false;
                    }
                }
            }
        }

        if input.left {
            piece.column -= 1;
        } else if input.right {
            piece.column += 1;
        }

        /*
         * Vertical collision detection
         */
        'collision_vertical: for (row_index, piece_row) in piece.current_shape().iter().enumerate()
        {
            let y = (piece.row + row_index) * COLUMN_COUNT;
            for (col_index, block) in piece_row.iter().enumerate() {
                if *block == 0 {
                    continue;
                }
                let x = piece.column + col_index as i32;

                /*
                 * check for things below this shape. We cheat here and wait until we're *actually* in the block below
                 * to allow the player to slide the block around for the duration of a single 'tick' even while it's
                 * touching something below it
                 */
                let current_row_index = x as usize + y;
                if current_row_index < BLOCK_COUNT && blocks[current_row_index] == Color::BLACK {
                    continue;
                }

                // we've hit either the bottom of the playfield or a block underneath us

                // copy all the piece blocks into the background blocks array
                for (row_index, piece_row) in piece.current_shape().iter().enumerate() {
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
                for (row, blocks) in blocks.chunks(COLUMN_COUNT).enumerate() {
                    let filled_blocks = blocks
                        .iter()
                        .filter(|&colour| *colour != Color::BLACK)
                        .count();
                    if filled_blocks == COLUMN_COUNT {
                        lines.push(row);
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

                println!("Score: {:?}", score);

                // as soon as we hit anything, spawn a new piece and don't look for any further collisions
                piece = next_piece;
                next_piece = Piece::random();
                break 'collision_vertical;
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
            let x = (PLAYFIELD_START_X + ((i % 10) * BLOCK_SIZE)) as i32;
            let y = (PLAYFIELD_START_Y + ((i / 10) * BLOCK_SIZE)) as i32;
            let size = BLOCK_SIZE as u32;
            let inner = if colour != Color::BLACK {
                (1, 2)
            } else {
                (0, 0)
            };
            let inner_rect = Rect::new(x + inner.0, y + inner.0, size - inner.1, size - inner.1);

            canvas.set_draw_color(Color::GREY);
            canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();

            canvas.set_draw_color(colour);
            canvas.fill_rect(inner_rect).unwrap();
        }

        render_piece(&mut canvas, &piece, PLAYFIELD_START_X, PLAYFIELD_START_Y);
        render_piece(&mut canvas, &next_piece, 500, 80);
        canvas.present();

        let sleep_ms = (1000 / 30) - frame_start.elapsed().as_millis().min(0);
        thread::sleep(Duration::from_millis(sleep_ms as u64));
    }
}