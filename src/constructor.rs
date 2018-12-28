#![allow(unused_variables)]
#![allow(dead_code)]

use new_index::_Index;

use color::Color;
use std::collections::HashMap;          
use square::Square;
use recorder::Move;
pub struct Constructor
{
    knights: Vec<HashMap<Square, _Index<Color>>>,
    rooks: Vec<HashMap<Square, _Index<Color>>>,
    bishops: Vec<HashMap<Square, _Index<Color>>>,
    queens: Vec<(Square, _Index<Color>)>,
    kings: Vec<(Square, _Index<Color>)>,
    pawns: Vec<HashMap<Square, _Index<Color>>>,
    moves: Option<Vec<Move>>


}

impl Constructor
{
    pub fn new() -> Self
    {
        let mut knigts = Vec::with_capacity(2);
        let mut knight_white = HashMap::with_capacity(2);
        let mut knight_black = HashMap::with_capacity(2);

        let s1 = Square::new(1, 7);
        let s2 = Square::new(6, 7);

        knight_white.insert(s1, _Index::Knight(Color::White));
        knight_white.insert(s2, _Index::Knight(Color::White));

        let s1 = Square::new(1, 0);
        let s2 = Square::new(6, 0);

        knight_black.insert(s1, _Index::Knight(Color::Black));
        knight_black.insert(s2, _Index::Knight(Color::Black));

        knigts[0] = knight_white;
        knigts[1] = knight_black;
        
        let mut rooks = Vec::with_capacity(2);
        let mut rooks_white = HashMap::with_capacity(2);
        let mut rooks_black = HashMap::with_capacity(2);

        let s1 = Square::new(0, 7);
        let s2 = Square::new(7, 7);

        rooks_white.insert(s1, _Index::Rook(Color::White));
        rooks_white.insert(s2, _Index::Rook(Color::White));

        let s1 = Square::new(0, 0);
        let s2 = Square::new(7, 0);

        rooks_black.insert(s1, _Index::Rook(Color::Black));
        rooks_black.insert(s2, _Index::Rook(Color::Black));

        rooks[0] = rooks_white;
        rooks[1] = rooks_black;

        let mut bishops = Vec::with_capacity(2);
        let mut bishops_white = HashMap::with_capacity(2);
        let mut bishops_black = HashMap::with_capacity(2);

        let s1 = Square::new(2, 7);
        let s2 = Square::new(5, 7);

        bishops_white.insert(s1, _Index::Bishop(Color::White));
        bishops_white.insert(s2, _Index::Bishop(Color::White));

        let s1 = Square::new(2, 0);
        let s2 = Square::new(2, 0);

        bishops_black.insert(s1, _Index::Bishop(Color::Black));
        bishops_black.insert(s2, _Index::Bishop(Color::Black));

        bishops[0] = bishops_white;
        bishops[1] = bishops_black;


        let mut queens = Vec::with_capacity(2);
        let q1 = (Square::new(3, 7), _Index::Queen(Color::White));
        let q2 = (Square::new(3, 0), _Index::Queen(Color::Black));
        queens[0] = q1;
        queens[1] = q2;
        
        let mut kings = Vec::with_capacity(2);
        let k1 = (Square::new(4, 7), _Index::King(Color::White));
        let k2 = (Square::new(4, 0), _Index::King(Color::Black));
        kings[0] = k1;
        kings[1] = k2;

        let mut pawns = Vec::with_capacity(2);
        let mut pawns_white = HashMap::with_capacity(7);
        let mut pawns_black = HashMap::with_capacity(7);

        for i in 0..7
        {
            pawns_white.insert( Square::new(i, 1), _Index::Pawn(Color::White) );
            pawns_black.insert( Square::new(i, 6), _Index::Pawn(Color::Black) );
        }
        pawns[0] = pawns_white;
        pawns[1] = pawns_black;


        Constructor {
            knights: knigts,
            rooks: rooks,
            bishops: bishops,
            kings: kings,
            queens: queens,
            pawns: pawns,
            moves: None
        }
    }

    pub fn parse_move(&mut self, notation: &str) -> Move
    {
        let mut iter = notation.chars();
        let letter = iter.next().unwrap();
        match letter
        {
            'N' => self.knight(iter),
            'B' => self.bishop(iter),
            'R' => self.rook(iter),
            'Q' => self.queen(iter),
            'K' => self.king(iter),
            
            _ => self.pawn(letter, iter),
        }
    }

    fn knight(&mut self, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }
    fn bishop(&mut self, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }
    fn king(&mut self, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }
    fn queen(&mut self, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }
    fn rook(&mut self, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }
    fn pawn(&mut self, letter: char, iter: std::str::Chars) -> Move
    {
        unimplemented!()
    }

    fn get_index(&self, square: &Square) -> _Index<Color>
    {
        unimplemented!() 
    }


}
