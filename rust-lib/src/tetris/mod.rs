#![feature(duration_as_u128)]

extern crate sdl2;

mod constants;
mod game;
mod game_color;
mod others;
mod piece;
mod renderer;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

use self::constants::*;
use self::game::Game;
use self::game_color::GameColor;
use self::piece::Piece;
use self::renderer::{create_texture_rect, create_window, draw_map, draw_piece};

use self::constants::set_dimension;

// initialize sdl context and canvas
pub(crate) fn main(width: u32, height: u32) {
    set_dimension(width, height);
    let (sdl_ctx, mut canvas) = create_window(width, height);

    let mut event_pump = sdl_ctx
        .event_pump()
        .expect("Failed to get sdl context event pump");

    let texture_creator = canvas.texture_creator();

    macro_rules! texture {
        ($color: expr) => {
            create_texture_rect(&mut canvas, &texture_creator, $color)
                .expect("Failed to create texture");
        };
    }

    // generate all the textures needed later on at once
    let textures = [
        texture!(GameColor::Red),
        texture!(GameColor::Green),
        texture!(GameColor::Blue),
        texture!(GameColor::Yellow),
        texture!(GameColor::Cyan),
        texture!(GameColor::Orange),
        texture!(GameColor::Purple),
        texture!(GameColor::Gray),
        texture!(GameColor::Pink),
    ];

    let mut game = Game::new();
    let mut last_instant = Instant::now();

    // loop till we receive exit signal QUIT/ESCAPE key
    loop {
        let mut p = game.current_piece.unwrap();

        if last_instant.elapsed().as_millis() > LEVEL_TIMES[game.current_level] as u128 {
            if !p.move_position(&game.map, p.x, p.y + 1) {
                game.finalize_move(&mut p);
            }

            last_instant = Instant::now();
        }
        game.current_piece = Some(p);

        handle_events(&mut game, &mut event_pump);
        render_scene(&mut canvas, &textures, &game);

        if game.quit {
            break;
        }
    }
}

fn handle_events(game: &mut Game, event_pump: &mut EventPump) {
    use sdl2::event::Event::{FingerDown, FingerUp, KeyDown, Quit};
    use sdl2::keyboard::Keycode::*;

    let (s_width, s_height) = get_dimension();
    let s_width = s_width as f32;
    let s_height = s_height as f32;

    let mut p = game.current_piece.unwrap();
    let (mut dx, mut dy) = (0, 0);

    for event in event_pump.poll_iter() {
        match event {
            Quit { .. }
            | KeyDown {
                keycode: Some(Escape),
                ..
            } => {
                game.quit = true;
            }
            KeyDown {
                keycode: Some(Left),
                ..
            } => {
                dx -= 1;
            }
            KeyDown {
                keycode: Some(Right),
                ..
            } => {
                dx += 1;
            }
            KeyDown {
                keycode: Some(Up), ..
            } => {
                p.rotate(&game.map);
            }
            KeyDown {
                keycode: Some(Down),
                ..
            } => {
                dy += 1;
            }
            KeyDown {
                keycode: Some(Space),
                ..
            } => {
                while p.move_position(&game.map, p.x, p.y + 1) {}
                game.finalize_move(&mut p);
            }
            KeyDown {
                keycode: Some(N), ..
            } => {
                p = Piece::random();
            }
            KeyDown {
                keycode: Some(F), ..
            } => {
                game.finalize_move(&mut p);
            }
            FingerDown { x, y, .. } => {
                game.now = Instant::now();
                if game.check_piece_to_rotate(x * s_width, y * s_height) {
                    // rotate
                    p.rotate(&game.map);
                    continue;
                }
            }
            FingerUp { x, y, .. } => {
                let x = (x * NUM_BLOCKS_X as f32).floor();
                let y = (y * NUM_BLOCKS_Y as f32).floor();

                let haft_size_x = (NUM_BLOCKS_X / 2) as f32;

                if x < haft_size_x {
                    // move left
                    dx -= 1;
                    continue;
                }

                if x > haft_size_x {
                    // move right
                    dx += 1;
                    continue;
                }
            }
            _ => {}
        }
    }

    p.move_position(&game.map, p.x + dx, p.y + dy);
    game.current_piece = Some(p);
}

fn render_scene(mut canvas: &mut Canvas<Window>, textures: &[Texture; 9], game: &Game) {
    // set canvas background and clear it
    canvas.set_draw_color(GameColor::Gray);
    canvas.clear();

    draw_map(&mut canvas, &textures, &game.map);
    draw_piece(
        &mut canvas,
        &textures,
        &game.current_piece.unwrap(),
        &game.get_shadow_piece(),
    );

    canvas.present();

    sleep(Duration::new(0, 1_000_000_000u32 / 60)); // for 60 fps TODO: use better time sync
}
