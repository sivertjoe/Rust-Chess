extern crate regex;
use self::regex::Regex;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{self, BufRead}; 
use game::Game;
use constructor::Constructor;

unsafe impl Sync for Input{}

pub struct Input
{
    send: Sender<String>, 
    recv: Receiver<String>,
    has_started: bool,
}


impl Input
{
    pub fn new() -> Input
    {
        let channel = channel();
            Input {
                send: channel.0,
                recv: channel.1,
                has_started: false,
            }
    
    }

    pub fn init(&mut self)
    {
        // initing more than once is baaaaaad
        if !self.has_started
        {
            let sender = self.send.clone();
            thread::spawn(move ||
            {
                loop
                {
                    let mut line = String::new();
                    let stdin = io::stdin();
                    stdin.lock().read_line(&mut line).expect("Could not read line");
                    sender.send(line).unwrap();
                }
            });
            self.has_started = true;
        }
    }

    pub fn update(&self, game: &mut Game)
    {
        match self.recv.try_recv()
        {
            Ok(v) => self.handle_input(v, game),
            _ => {},
        };
    }

    fn handle_input(&self, input: String, _game: &mut Game)
    {
        let mut input = input.split_whitespace();
        match input.next()
        {
            Some("import") => 
            {
                if let Some(path) = input.next()
                {
                    self.import_game(_game, path);
                }
            },
            _ => return,
        }
    }

    fn import_game(&self, game: &mut Game, path: &str)
    {
        use std::path::Path;
        use std::fs::File;
        use std::io::prelude::*;
        let path = Path::new(&path); 
        
        let mut file = match File::open(path)
        {
            Ok(f) => f,
            _ => return,
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        
        let pattern = r"([NBRQK]?[abcdefgh]?[12345678]?x?[abcdefg][12345678]\+?)";
        let regex = Regex::new(&pattern).unwrap();
        let mut moves = Vec::new();
        
        let mut constructor = Constructor::new();

        for cap in regex.captures_iter(&content)
        {
            moves.push(constructor.parse_move(&cap[0]));    
        }
   
        for _ in 0..game.recorder.n_moves()
        {
            game.recorder._undo();
        }
        game.recorder.set_moves(moves);
    }
}
