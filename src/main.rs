#![feature(nll, vec_remove_item)]
extern crate sfml;

use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{RenderWindow};
use std::env::args;
#[allow(non_snake_case)]

mod board;
mod game;
mod pieces;
mod resources;
mod square;
mod new_piece_creator;
mod color;
mod utility;
mod recorder;
mod new_index;

mod constructor;
mod mov_functions;
mod input;
mod highlight;
mod arrow;
mod angle;
mod temp_move;

use new_index::*;
use game::{Game, init_recourse};
use resources::Resources;
use input::Input;

const DEFAULT_DIMENTIONS: (u32, u32) = (500, 500);
pub type KEY = _Index<color::Color>;



fn main() 
{
    let size = size_args_or_default(DEFAULT_DIMENTIONS);
    let mut window = init_window(size);    
    window.set_vertical_sync_enabled(true);
    
    let mut res: Resources<KEY> = Resources::new();
    init_recourse(&mut res);

    let mut game = Game::new(&res, &window);
   
    let input = Input::new();

    while window.is_open()
    {

        handle_events(&mut window);
        input.update(&mut game);

        game.update(&mut window);
        game.display(&mut window);

        window.display();
    }
}

fn handle_events(window: &mut RenderWindow)
{
    for event in window.poll_event()
    {
        match event
        {
            Event::Closed => window.close(),
            _ => {},
        };
    }
}

fn init_window(size: (u32, u32)) -> RenderWindow
{
    RenderWindow::new(
        VideoMode::new(size.0, size.1, VideoMode::desktop_mode().bits_per_pixel),
        "Chess",
        Style::CLOSE,
        &Default::default())
}


fn size_args_or_default(default: (u32, u32)) -> (u32, u32)
{
    match (args().nth(1), args().nth(2))
    {
        (Some(s1), Some(s2)) => match (s1.parse(), s2.parse())
            {
                (Ok(val1), Ok(val2)) => (val1, val2),
                _ => default,
            },
        _ => default,
    }
}
