extern crate sdl2;

use std::thread;
use std::time::{Duration, Instant};

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
const BLOCKS_PER_ROW: usize = 10;
const ROW_COUNT: usize = 20;
const BLOCK_COUNT: usize = BLOCKS_PER_ROW * ROW_COUNT;
const COLOR_ORANGE: Color = Color::RGBA(255, 165, 0, 255);
const EMPTY: Color = Color::BLACK;

// rustfmt does funny things with the Tetromino formatting below :/
// each line represents all N rows of a given piece at a given angle
// based on: https://vignette.wikia.nocookie.net/tetrisconcept/images/3/3d/SRS-pieces.png
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
    shapes: ShapeSet,
    angle: usize,
    column: i32,
    row: usize,
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

macro_rules! shape_list {
    ($a:expr) => {
        $a.map(|p| p.map(|p| p.to_vec()).to_vec()).to_vec()
    };
}

type Shape = Vec<Vec<u8>>;
type ShapeList = Vec<Shape>;
type ShapeWidths = [u8; 2];
struct ShapeSet(ShapeList, ShapeWidths, Color);

impl ShapeSet {
    pub fn from_index(piece_type: usize) -> ShapeSet {
        match piece_type {
            0 => ShapeSet(shape_list!(T_PIECES), [3, 2], Color::MAGENTA),
            1 => ShapeSet(shape_list!(O_PIECES), [2, 2], Color::YELLOW),
            2 => ShapeSet(shape_list!(S_PIECES), [3, 2], Color::GREEN),
            3 => ShapeSet(shape_list!(Z_PIECES), [3, 2], Color::RED),
            4 => ShapeSet(shape_list!(L_PIECES), [3, 2], COLOR_ORANGE),
            5 => ShapeSet(shape_list!(J_PIECES), [3, 2], Color::BLUE),
            _ => ShapeSet(shape_list!(I_PIECES), [4, 1], Color::CYAN),
        }
    }
    pub fn get(&self, angle: usize) -> Shape {
        self.0[angle].clone()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn colour(&self) -> Color {
        self.2
    }
    pub fn width(&self, angle: usize) -> u8 {
        self.1[angle % 2]
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

    pub fn current_shape(&self) -> Shape {
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

    pub fn draw(&self, canvas: &mut Canvas<Window>, sx: usize, sy: usize) {
        for (col_index, row_index) in self.block_iter() {
            let x = sx as i32 + ((self.column + col_index as i32) * BLOCK_SIZE as i32);
            let y = (sy + ((self.row + row_index) * BLOCK_SIZE)) as i32;
            let size = BLOCK_SIZE as u32;
            canvas.set_draw_color(Color::GREY);
            canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();

            canvas.set_draw_color(self.shapes.colour());
            canvas
                .fill_rect(Rect::new(x + 1, y + 1, size - 2, size - 2))
                .unwrap();
        }
    }

    pub fn block_iter(&self) -> BlockIterator {
        BlockIterator(0, 0, self.current_shape())
    }
}

struct BlockIterator(usize, usize, Shape);

impl Iterator for BlockIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.0;
        let y = self.1;
        if y == self.2.len() {
            None
        } else {
            let row = &self.2[y];
            self.0 += 1;
            if self.0 == row.len() {
                self.0 = 0;
                self.1 += 1;
            }
            if row[x] == 0 {
                self.next() // don't return empty blocks
            } else {
                Some((x, y))
            }
        }
    }
}

fn main() {
    let mut blocks = [EMPTY; BLOCK_COUNT];
    let mut piece = Piece::random();
    let mut next_piece = Piece::random();
    let mut last_rotation = Instant::now();
    let mut tick_time = Instant::now();
    let mut score = Score {
        points: 0,
        lines: 0,
    };

    // SDL2 display/event handling
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("tetrus", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    /*
     * @TODO:
     * - score display
     * - space bar support
     * - sound
     */
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

            // bring the piece x value back in bounds to prevent overflow
            let max = (BLOCKS_PER_ROW - piece.width() as usize) as i32;
            piece.column = piece.column.min(max).max(0);
        }

        /*
         * Horizontal collision detection
         * This can unset any left/right movement intended by the player
         */
        for (col_index, row_index) in piece.block_iter() {
            let x = piece.column + col_index as i32;
            let y = (piece.row + row_index) * BLOCKS_PER_ROW;

            // playfield left/right
            if x <= 0 {
                input.left = false;
            } else if x >= BLOCKS_PER_ROW as i32 - 1 {
                input.right = false;
            } else {
                // neighbouring blocks
                // bit of a bodge - only assigned here because without first testing x we might overflow
                let left_neighbour = x as usize + y - 1;
                let right_neighbour = x as usize + y + 1;
                if left_neighbour >= BLOCK_COUNT || blocks[left_neighbour] != EMPTY {
                    input.left = false;
                }
                if right_neighbour >= BLOCK_COUNT || blocks[right_neighbour] != EMPTY {
                    input.right = false;
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
        for (col_index, row_index) in piece.block_iter() {
            let x = piece.column + col_index as i32;
            let y = (piece.row + row_index) * BLOCKS_PER_ROW;

            /*
             * check for things below this shape. We cheat here and wait until we're actually *in* the block below
             * This allows the player to slide the block around for the duration of a single tick while it's
             * touching something below it
             */
            let current_index = x as usize + y;
            if current_index < BLOCK_COUNT && blocks[current_index] == EMPTY {
                continue;
            }

            // we've hit either the bottom of the playfield or a block underneath us
            // copy all the piece blocks into the background blocks array
            for (col_index, row_index) in piece.block_iter() {
                // the -1 here is very important. As soon as anything goes off screen or collides with something below it
                // we need to snap it back up a row into place
                let x = piece.column + col_index as i32;
                let y = (piece.row + row_index - 1) * BLOCKS_PER_ROW;
                blocks[x as usize + y] = piece.shapes.colour();
            }

            let mut cleared_lines = 0;
            for start in (0..BLOCK_COUNT).step_by(BLOCKS_PER_ROW) {
                let end = start + BLOCKS_PER_ROW;
                let filled = &blocks[start..end].iter().filter(|&c| *c != EMPTY).count();
                if *filled == BLOCKS_PER_ROW {
                    // this will memmove everything up until the start of the cleared line 'down' by one line
                    // which has the effect of replacing (or clearing) the line with whatever was above it
                    blocks.copy_within(0..start, BLOCKS_PER_ROW);
                    cleared_lines += 1;
                }
            }
            score.lines += cleared_lines;
            score.points += (cleared_lines * cleared_lines) * 100;

            println!("Score: {:?}", score);

            // as soon as we hit anything, spawn a new piece and don't look for any further collisions
            piece = next_piece;
            next_piece = Piece::random();
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
            let inner = if colour != EMPTY { (1, 2) } else { (0, 0) };
            let inner_rect = Rect::new(x + inner.0, y + inner.0, size - inner.1, size - inner.1);

            canvas.set_draw_color(Color::GREY);
            canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();

            canvas.set_draw_color(colour);
            canvas.fill_rect(inner_rect).unwrap();
        }

        piece.draw(&mut canvas, PLAYFIELD_START_X, PLAYFIELD_START_Y);
        next_piece.draw(&mut canvas, 500, 80);
        canvas.present();

        let sleep_ms = (1000 / 30) - frame_start.elapsed().as_millis().min(0);
        thread::sleep(Duration::from_millis(sleep_ms as u64));
    }
}
