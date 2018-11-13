extern crate sfml;

use sfml::window::{VideoMode, Style, Event};

use sfml::graphics::{RenderWindow, RenderTarget, Color};
use std::env::args;
#[allow(non_snake_case)]



mod board;
mod game;
mod pieces;
mod recourses;
mod square;
mod piece_creator;
mod index;
mod color;

use index::Index;

use game::{Game, init_recourse};
use recourses::Recourses;


const DEFAULT_DIMENTIONS: (u32, u32) = (500, 500);

fn main() 
{
     /*
     * Set dimentions to args, if any
     */
    let size = size_args_or_default(DEFAULT_DIMENTIONS);
    let mut window = init_window(size);    
    
    let mut res: Recourses<Index> = Recourses::new();
    init_recourse(&mut res);

    let game = Game::new(&mut res, &window);
    
   
    while window.is_open()
    {
        for event in window.poll_event()
        {
            match event
            {
                Event::Closed => window.close(),
                _ => {},
            };
        }
        window.clear(&Color::BLACK);

        game.update(&mut window);
        
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
