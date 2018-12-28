#![feature(nll)]
extern crate sfml;

use sfml::window::{VideoMode, Style, Event};
use sfml::window::mouse::Button;
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
    
    let mut res: Resources<KEY> = Resources::new();
    init_recourse(&mut res);

    let mut game = Game::new(&res, &window);
   
    let mut input = Input::new();
    input.init();


    while window.is_open()
    {
        for event in window.poll_event()
        {
            match event
            {
                Event::Closed => window.close(),
                
                Event::MouseButtonReleased{ button: Button::Left, ..} =>
                    game.hold_mouse = false,
                
                Event::MouseButtonPressed{ button: Button::Left, .. } => 
                    game.hold_mouse = true,

                _ => {},
            };
        }
        input.update(&mut game);

        game.update(&mut window);
        game.display(&mut window);

        window.display();
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
