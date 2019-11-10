use crate::tetris::constants::*;
use crate::tetris::others::{PieceType, Presence};
use crate::tetris::piece::Piece;
use rand::random;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type GameMap = Vec<Vec<Presence>>;

pub struct Game {
    pub current_piece: Option<Piece>,
    pub score: usize,
    pub lines_cleared: usize,
    pub current_level: usize,
    pub map: Vec<Vec<Presence>>,
    pub quit: bool,
    pub now: Instant,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_piece: Some(Piece::from(random::<PieceType>())),
            score: 0,
            lines_cleared: 0,
            current_level: 0,
            map: vec![vec![Presence::No; NUM_BLOCKS_X]; NUM_BLOCKS_Y],
            quit: false,
            now: Instant::now(),
        }
    }

    pub fn check_piece_to_rotate(&self, x: f32, y: f32) -> bool {
        let offset_top = get_offset_top() as f32;
        let y = y - offset_top;

        if let Some(piece) = self.current_piece {
            let texture_size = TEXTURE_SIZE as isize;
            let state = piece.get_block_matrix(piece.current_state);
            for dx in 0..4 {
                for dy in 0..4 {
                    let cell = state[dy][dx];
                    if cell != Presence::No {
                        let dx = piece.x + dx as isize;
                        let dy = piece.y + dy as isize;

                        if x >= (dx * texture_size) as f32
                            && x <= ((dx + 1) * texture_size) as f32
                            && y >= (dy * texture_size) as f32
                            && y <= ((dy + 1) * texture_size) as f32
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn check_lines(&mut self) {
        let mut to_clear = vec![];

        for y in 0..NUM_BLOCKS_Y {
            if self.map[y].iter().all(|&x| x != Presence::No) {
                to_clear.push(y);
            }
        }

        self.increase_lines(to_clear.len());

        for index in to_clear.into_iter() {
            self.map.remove(index);
            self.map.insert(0, vec![Presence::No; NUM_BLOCKS_X]);
        }
    }

    pub fn finalize_move(&mut self, piece: &mut Piece) {
        if piece.y < 0 {
            self.quit = true;
        } else {
            piece.freeze(&mut self.map);
            self.check_lines();
            *piece = Piece::random();
        }
    }

    fn increase_lines(&mut self, delta: usize) {
        self.lines_cleared += delta;
        if self.lines_cleared > LEVEL_LINES[self.current_level] {
            self.current_level = usize::max(self.current_level + 1, NUM_LEVELS - 1)
        }
    }

    pub fn get_shadow_piece(&self) -> Piece {
        let mut p = self.current_piece.unwrap().clone();
        while p.move_position(&self.map, p.x, p.y + 1) {}
        p
    }
}
