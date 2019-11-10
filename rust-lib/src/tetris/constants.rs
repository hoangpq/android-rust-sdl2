pub const TITLE: &'static str = "Rust SDL2";

pub const NUM_BLOCKS_X: usize = 10;
pub const NUM_BLOCKS_Y: usize = 18;

pub const BORDER_WIDTH: u32 = 1;
pub const TEXTURE_SIZE: u32 = 80;
pub const TEXTURE_SIZE_INNER: u32 = TEXTURE_SIZE - BORDER_WIDTH * 2;

pub const WIDTH: u32 = NUM_BLOCKS_X as u32 * TEXTURE_SIZE;
pub const HEIGHT: u32 = NUM_BLOCKS_Y as u32 * TEXTURE_SIZE;

pub const NUM_LEVELS: usize = 10;
pub const LEVEL_TIMES: [usize; NUM_LEVELS] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
pub const LEVEL_LINES: [usize; NUM_LEVELS] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct SDL2Config {
    width: u32,
    height: u32,
    offset_top: u32,
}

impl SDL2Config {
    pub fn new() -> Self {
        SDL2Config::default()
    }
    pub fn set_dimension(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.offset_top = height - HEIGHT;
    }
    pub fn get_dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn get_offset_top(&self) -> u32 {
        self.offset_top
    }
}

lazy_static! {
    static ref SDL_CONFIG: Arc<Mutex<SDL2Config>> = Arc::new(Mutex::new(SDL2Config::new()));
}

pub fn set_dimension(width: u32, height: u32) {
    let config = Arc::clone(&SDL_CONFIG);
    let mut config = config.lock().unwrap();
    config.set_dimension(width, height);
}

pub fn get_dimension() -> (i32, i32) {
    let config = Arc::clone(&SDL_CONFIG);
    let config = config.lock().unwrap();
    let (width, height) = config.get_dimension();
    (width as i32, height as i32)
}

pub fn get_offset_top() -> u32 {
    let config = Arc::clone(&SDL_CONFIG);
    let config = config.lock().unwrap();
    config.get_offset_top()
}
