use new_index::_Index;
use std::collections::HashSet;

use color::Color;
use std::collections::HashMap;          
use square::Square;
use r#move::Move;

pub struct Constructor
{
    knights: Vec<HashMap<Square, _Index<Color>>>,
    rooks: Vec<HashMap<Square, _Index<Color>>>,
    bishops: Vec<HashMap<Square, _Index<Color>>>,
    queens: Vec<(Square, _Index<Color>)>,
    kings: Vec<(Square, _Index<Color>)>,
    pawns: Vec<HashMap<Square, _Index<Color>>>,
}

fn create_officers(
        (x1, x2): (u8, u8),
        (index1, type1_, y1): (usize, _Index<Color>, u8),
        (index2, type2_, y2): (usize, _Index<Color>, u8))
-> Vec<HashMap<Square, _Index<Color>>>
{
    let mut vec = Vec::with_capacity(2);
    let mut officer_white = HashMap::with_capacity(2);
    let mut officer_black = HashMap::with_capacity(2);

    let s1 = Square::new(x1, y1);
    let s2 = Square::new(x2, y1);
    
    officer_white.insert(s1, type1_.clone());
    officer_white.insert(s2, type1_);
    

    let s1 = Square::new(x1, y2);
    let s2 = Square::new(x2, y2);
    
    officer_black.insert(s1, type2_.clone());
    officer_black.insert(s2, type2_);


    vec.insert(index1, officer_white);
    vec.insert(index2, officer_black);
    vec
}


fn create_royals(
            x: u8, 
            (index1, type1_, y1): (usize, _Index<Color>, u8), 
            (index2, type2_, y2): (usize, _Index<Color>, u8)) 
-> Vec<(Square, _Index<Color>)>
{
   let mut vec = Vec::with_capacity(2);
   let q1 = (Square::new(x, y1), type1_);
   let q2 = (Square::new(x, y2), type2_);

   vec.insert(index1, q1);
   vec.insert(index2, q2);
   vec
}

fn create_pawns() -> Vec<HashMap<Square, _Index<Color>>>
{
    let mut pawns = Vec::with_capacity(2);
    let mut pawns_white = HashMap::with_capacity(7);
    let mut pawns_black = HashMap::with_capacity(7);

    for i in 0..=7
    {
        pawns_white.insert( Square::new(i, 6), _Index::Pawn(Color::White) );
        pawns_black.insert( Square::new(i, 1), _Index::Pawn(Color::Black) );
    }
    pawns.insert(0, pawns_white);
    pawns.insert(1, pawns_black);

    pawns
}

impl Constructor
{
    pub fn new() -> Self
    {
        let knigts = create_officers(
                    (1, 6),
                    (0, _Index::Knight(Color::White), 7),
                    (1, _Index::Knight(Color::Black), 0));
                                        
        let rooks = create_officers(
                    (0, 7),
                    (0, _Index::Rook(Color::White), 7),
                    (1, _Index::Rook(Color::Black), 0));
        let bishops = create_officers(
                    (2, 5),
                    (0, _Index::Bishop(Color::White), 7),
                    (1, _Index::Bishop(Color::Black), 0));

        let queens = create_royals(
                     3, 
                    (0, _Index::Queen(Color::White), 7), 
                    (1, _Index::Queen(Color::Black), 0));
        
        let  kings = create_royals(
                     4, 
                    (0, _Index::King(Color::White), 7), 
                    (1, _Index::King(Color::Black), 0));

        let pawns = create_pawns();

        Constructor {
            knights: knigts,
            rooks: rooks,
            bishops: bishops,
            kings: kings,
            queens: queens,
            pawns: pawns,
        }
    }

    pub fn parse_move(&mut self, notation: &str, color: Color) -> Move
    {
        if notation.starts_with("O")
        {
                return self.castle(notation.len(), color);
        }
        let mut iter = notation.chars();
        let letter = iter.next().unwrap();
        match letter
        {
            'N' => self.knight(iter, color),
            'B' => self.bishop(iter, color),
            'R' => self.rook(iter, color),
            'Q' => self.queen(iter, color),
            'K' => self.king(iter, color),
            
            _ => self.pawn(letter, iter, color),
        }
    }

    fn castle(&mut self, length: usize, color: Color) -> Move
    {
            //0-0: 3, 0-0-0: 5
        let row = match &color { &Color::White => 7, _ => 0 };
        if length == 3
        {
            let from = Square::new(4, row);
            let to = Square::new(6, row);
            self.fix_move(from.clone(), to.clone(), _Index::King(color.clone()));

            let _to = Square::new(5, row);
            let _from = Square::new(7, row);
            self.fix_move(_from.clone(), _to.clone(), _Index::Rook(color.clone()));

            return Move {
                piece: _Index::King(color),
                from: from,
                to: to,
                capture: None,
            };
        }
        let from = Square::new(4, row);
        let to = Square::new(2, row);
        self.fix_move(from.clone(), to.clone(), _Index::King(color.clone()));

        let _to = Square::new(4, row);
        let _from = Square::new(0, row);
        self.fix_move(_from.clone(), _to.clone(), _Index::Rook(color.clone()));
        
        Move {
            piece: _Index::King(color),
            from: Square::new(4, row),
            to: Square::new(2, row),
            capture: None
        }
    }    

    fn knight(&mut self, mut iter: std::str::Chars, color: Color) -> Move
    {
        // possible moves:
        // Ne4, Nxe4, N3e4, N3xe4, Nde4, Ndxe4
        let notation = iter.as_str();
        let token = iter.next().unwrap();



        let n = match &color { Color::White => 0, _ => 1};

        if token == 'x' 
        {
            let from = self.find_knight( Square::from_nontation(iter.as_str()), n );
            return self.knight_takes(iter, from, color); 
        }

        if token.is_numeric()
        {
            let index = 8 - (token as u8 - 48);
            let mut from_square = Square::new(9, 9);
            let notation = iter.as_str();
            for square in self.knights.get(n).unwrap().keys()
            {
                if square.row == index
                {
                    from_square.set(square.col, square.row);
                }
            }
            if iter.next().unwrap() == 'x'
            {
                assert!( from_square.row != 9 );
                return self.knight_takes(iter, from_square, color);
            }
            else
            {
                assert!( from_square.row != 9 );
                let to = Square::from_nontation(notation);
                self.fix_move(from_square.clone(), to.clone(), _Index::Knight(color.clone()));
                return Move
                {
                    piece: _Index::Knight(color),
                    to: to,
                    from: from_square,
                    capture: None,
                };
            }

        }
        if token.is_lowercase() && iter.as_str().len() >= 2
        {
            let file = token as u8 - 97;
            let mut from_square = Square::new(9, 9);
            let notation = iter.as_str();
            for square in self.knights.get(n).unwrap().keys()
            {
                if square.col == file
                {
                    from_square.set(square.col, square.row);
                }
            }
            if iter.next().unwrap() == 'x'
            {
                assert!( from_square.row != 9 );
                return self.knight_takes(iter, from_square, color);
            }
            else
            {
                assert!( from_square.row != 9 );
                let to = Square::from_nontation(notation);
                self.fix_move(from_square.clone(), to.clone(), _Index::Knight(color.clone()));
                return Move
                {
                    piece: _Index::Knight(color),
                    to: to,
                    from: from_square,
                    capture: None,
                };
            }

        }
        let to =  Square::from_nontation(notation);
        let from = self.find_knight(to.clone(), n);
        self.fix_move(from.clone(), to.clone(), _Index::Knight(color.clone()));
        Move {
            piece: _Index::Knight(color),
            to: to,
            from: from,
            capture: None
        }
    }
    fn knight_takes(&mut self, iter: std::str::Chars, from: Square, color: Color) -> Move
    {
    
        let square = Square::from_nontation(iter.as_str());
        let capture = self.get_capture(&square, &color); 
        self.fix_move(from.clone(), square.clone(), _Index::Knight(color.clone()));
        
        Move {
            piece: _Index::Knight(color),
            to: square,
            from: from,
            capture: capture
            }
    }
    fn find_knight(&self, to: Square, n: usize) -> Square
    {
        let knights = self.knights.get(n).unwrap();
        for square in knights.keys()
        {
            use utility;
            let mut diff = utility::square_diff(square, &to);
            diff = (diff.0.abs(), diff.1.abs());
            if diff.0 == 1 && diff.1 == 2 || diff.0 == 2 && diff.1 == 1
            {
                return square.clone(); 
            }
        }
        panic!("Didn't find a knight, should be possible..");
    }
    fn bishop(&mut self, mut iter: std::str::Chars, color: Color) -> Move
    {
        // Possible moves:
        // be4, bd4, (white, black), bxe4, bxd4
        let notation = iter.as_str();
        let token = iter.next().unwrap();
        if token == 'x'
        {
            return self.bishop_takes(iter.as_str(), color); 
        }
        let to = Square::from_nontation(notation);
        let from = self.find_bishop(&to, &color);
        self.fix_move(from.clone(), to.clone(), _Index::Bishop(color.clone()));
        Move {
            piece: _Index::Bishop(color),
            to: to,
            from: from,
            capture: None,
        }
    }

    fn find_bishop(&self, final_square: &Square, color: &Color) -> Square
    {
        let mut from = Square::new(9, 9);
        let n = self.get_n(color);
        let sum = (final_square.col + final_square.row) % 2;
        self.bishops.get(n).unwrap().keys().for_each(|p|
        {
            let p_sum = (p.col + p.row) % 2;
            if p_sum == sum
            {
                from.set( p.col, p.row );
            }
        });
        if from.col == 9
        {
            panic!("Didn't find bishop");
        }
        from
    }
    fn bishop_takes(&mut self, notation: &str, color: Color) -> Move
    {
        let n = match &color { &Color::White => 0, _ => 1 };

        let final_square = Square::from_nontation(notation);
        let sum = (final_square.col + final_square.row) % 2;
        
        let mut from = Square::new(9, 9);
        self.bishops.get(n).unwrap().keys().for_each(|p|
        {
            let p_sum = (p.col + p.row) % 2;
            if p_sum == sum
            {
                from.set( p.col, p.row );
            }
        });
        if from.col == 9
        {
            panic!("Didn't find bishop");
        }
        self.fix_move(from.clone(), final_square.clone(), _Index::Bishop(color.clone())); 
        let capture = self.get_capture(&final_square, &color); 
        Move {
            piece: _Index::Bishop(color),
            to: final_square,
            from: from,
            capture: capture
        }

    }
    fn king(&mut self, mut iter: std::str::Chars, color: Color) -> Move
    {
        let n = match &color { &Color::White => 0, _ => 1 };
        let king = self.kings.get(n).unwrap();

        let from = king.0.clone();
        let notation = iter.as_str();
        
        if iter.next().unwrap() == 'x'
        {
            let to = Square::from_nontation(iter.as_str());
            let capture = self.get_capture(&to, &color);
            self.fix_move(from.clone(), to.clone(), _Index::King(color.clone()));
            return Move {
                piece: _Index::King(color),
                from: from,
                to: to,
                capture: capture
            };
        }
        let to = Square::from_nontation(notation);
        self.fix_move(from.clone(), to.clone(), _Index::King(color.clone()));
        Move {
            piece: _Index::King(color),
            from: from,
            to: to,
            capture: None
        }
    }
    fn queen(&mut self, mut iter: std::str::Chars, color: Color) -> Move
    {
        let n = self.get_n(&color);
        let queen = self.queens.get(n).unwrap();
        let from = queen.0.clone();
        let notation = iter.as_str();
        if iter.next().unwrap() == 'x'
        {
            let to = Square::from_nontation(iter.as_str());
            let capture = self.get_capture(&to, &color);
            self.fix_move(from.clone(), to.clone(), _Index::Queen(color.clone()));
            return Move {
                piece: _Index::Queen(color),
                from: from,
                to: to,
                capture: capture
            };
        }
        let to = Square::from_nontation(notation);
        self.fix_move(from.clone(), to.clone(), _Index::Queen(color.clone()));
        Move {
            piece: _Index::Queen(color),
            from: from,
            to: to,
            capture: None
        }
    }
    fn rook_takes(&mut self, notation: &str, color: Color, from: Square) -> Move
    {
            let to = Square::from_nontation(notation);
            let capture = self.get_capture(&to, &color);
            self.fix_move(from.clone(), to.clone(), _Index::Rook(color.clone()));

            Move {
                piece: _Index::Rook(color),
                to: to,
                from: from,
                capture: capture
            }
    }
    fn rook_move(&mut self, notation: &str, color: Color, from: Square) -> Move
    {
        let to = Square::from_nontation(notation);
        self.fix_move(from.clone(), to.clone(), _Index::Rook(color.clone()));

        Move {
            piece: _Index::Rook(color),
            to: to,
            from: from,
            capture: None
        }
    }
    fn rook(&mut self, mut iter: std::str::Chars, color: Color) -> Move
    {
        let notation = iter.as_str();
        let token = iter.next().unwrap();

        if token == 'x'
        {
            let from = self.find_rook(&Square::from_nontation(iter.as_str()), &color);
            return self.rook_takes(iter.as_str(), color, from);
        }

        if token.is_numeric()
        {
            let notation = iter.as_str();
            let from = self.find_rook_special(&color, token.to_digit(10), None);
            if iter.next().unwrap() == 'x'
            {
                return self.rook_takes(notation, color, from); 
            }
            return self.rook_move(notation, color, from);
        }

        if token.is_lowercase() && iter.as_str().len() >= 2
        {
            let notation = iter.as_str();
            let from = self.find_rook_special(&color, None, Some(token));
            if iter.next().unwrap() == 'x'
            {
                return self.rook_takes(notation, color, from); 
            }
            return self.rook_move(notation, color, from);
        }
        let to = Square::from_nontation(notation);
        let from = self.find_rook(&to, &color);
        return self.rook_move(notation, color, from);
    }
    fn find_rook_special(&self, color: &Color, number: Option<u32>, letter: Option<char>) -> Square
    {
        let n = self.get_n(color);
        if let Some(l) = letter
        {
            let file = l as u8 - 97;
            for key in self.rooks.get(n).unwrap().keys()
            {
                if key.col == file
                {
                    return key.clone();
                }
            }
            panic!("Didnt find rook (letter)");
        }

        if let Some(num) = number
        {
            let row = (8 - num) as u8;
            for key in self.rooks.get(n).unwrap().keys()
            {
                if key.row == row
                {
                    return key.clone();
                }
            }
            panic!("Didnt find rook (number)");
        }
        unimplemented!()
    }


    fn find_rook(&self, to: &Square, color: &Color) -> Square
    {
        let n = self.get_n(color);
        let rooks = self.rooks.get(n).unwrap();

        let mut iter = rooks.keys();
        let set = self.piece_set();
        while let Some(key) = iter.next()
        {
            use utility;
            let diff = utility::square_diff(to, key);
            if diff.0 == 0 && diff.1 != 0 || diff.1 == 0 && diff.0 != 0
            {
                let inc = match diff.0
                {
                    0 => (0, diff.1 / diff.1.abs()),
                    _ => (diff.0 / diff.0.abs(), 0),
                };
                let mut temp_square = key.clone();
                temp_square.inc(inc.0, inc.1);
                let mut flag = false;
                while &temp_square != to
                {
                    if set.contains(&temp_square) // If something blocks the path
                    {
                        flag = true;
                        break;
                    }
                    temp_square.inc(inc.0, inc.1);
                }
                if flag { continue };
                return key.clone();
            }
        }
        panic!("Didn't find rook");
    }


    fn pawn(&mut self, letter: char, mut iter: std::str::Chars, color: Color) -> Move
    {
        // Possible pawn moves:
        // e4, cxe4, en_passant: exd6
        let notation = format!("{}{}", letter, iter.as_str());
        let token = iter.next().unwrap();
        if token == 'x'
        {

            let to = Square::from_nontation(iter.as_str());
            let from = self.find_pawn(letter, true, iter.as_str(), &color);
            let capture = self.get_capture(&to, &color);
            self.fix_move(from.clone(), to.clone(), _Index::Pawn(color.clone()));
            return Move {
                piece: _Index::Pawn(color),
                to: to,
                from: from,
                capture: capture,
            };
        }
        let to = Square::from_nontation(notation.as_str());
        let from = self.find_pawn(letter, false, notation.as_str(), &color);
        self.fix_move(from.clone(), to.clone(), _Index::Pawn(color.clone()));

        Move {
            piece: _Index::Pawn(color),
            to: to,
            from: from,
            capture: None
        }
    }

    fn find_pawn(&mut self, letter: char, capture: bool,  notation: &str, color: &Color) -> Square
    {
        let to = Square::from_nontation(notation);
        let (n, _n) = match color { &Color::White => (0, 1), _ => (1, 0) };
        let inc = match color { Color::White => 1, _ => -1 };
        
        if capture
        {
            let row = 8 - (to.row as isize + inc ); 
            let not =  format!("{}{}", letter, row);
            return Square::from_nontation(not.as_str());

        }

        let pawns = self.pawns.get(n).unwrap();
        let mut one_down = Square::new(to.col, to.row as isize + inc);
        
        if pawns.get(&one_down).is_some()
        {
            return one_down;
        }

        match color
        {
            &Color::White => one_down.set(one_down.col, 6),
            _ => one_down.set(one_down.col, 1),
        };
        
        return one_down; // two_down
    }

    fn piece_set<'a>(&self) -> HashSet<Square>
    {
        let mut set: HashSet<Square> = HashSet::new();
        for k in self.pawns.get(0).unwrap().keys() { set.insert(k.clone()); }
        for k in self.pawns.get(1).unwrap().keys() { set.insert(k.clone()); }


        for k in self.rooks.get(0).unwrap().keys() { set.insert(k.clone()); }
        for k in self.rooks.get(1).unwrap().keys() { set.insert(k.clone()); }

        for k in self.knights.get(0).unwrap().keys() { set.insert(k.clone()); }
        for k in self.knights.get(1).unwrap().keys() { set.insert(k.clone()); }
        
        
        for k in self.bishops.get(0).unwrap().keys() { set.insert(k.clone()); }
        for k in self.bishops.get(1).unwrap().keys() { set.insert(k.clone()); }

        
        for k in self.queens.iter() { set.insert(k.0.clone()); }
        for k in self.kings.iter() { set.insert(k.0.clone()); }

        
        set
    }
    fn get_capture(&mut self, square: &Square, color: &Color) -> Option<_Index<Color>> 
    {
        /*        
        knights: Vec<HashMap<Square, _Index<Color>>>,
        rooks: Vec<HashMap<Square, _Index<Color>>>,
        bishops: Vec<HashMap<Square, _Index<Color>>>,
        queens: Vec<(Square, _Index<Color>)>,
        kings: Vec<(Square, _Index<Color>)>,
        pawns: Vec<HashMap<Square, _Index<Color>>>,
        */
        let n = match color { &Color::White => 1, _ => 0 };
        

        let pawns = self.pawns.get_mut(n).expect("paws");
        if pawns.get(square).is_some()
        {
            let pawn = pawns.remove(square);
            return pawn;
        }

        let qn = match self.queens.len() { 2 => n, _ => 0 };
        let queens = self.queens.get(qn).expect("queens get capture");
        if &queens.0 == square
        {
            let q = self.queens.remove(qn);
            return Some(q.1.clone());
        }

        let bishops = self.bishops.get_mut(n).expect("bishops get");
        if bishops.get(square).is_some()
        {
            return bishops.remove(square);
        }

        let rooks = self.rooks.get_mut(n).expect("rooks get");
        if rooks.get(square).is_some()
        {
            return rooks.remove(square);
        }
        let knights = self.knights.get_mut(n).expect("knights get");
        if knights.get(square).is_some()
        {
            return knights.remove(square);
        }
        // Only way to reach this point is for en passant
        /*let inc = match color { Color::White => 1, _ => -1 };
        let row = (square.row as isize + inc) as usize;
        let en_passant_square = Square::new(square.col, row);
        return pawns.remove(&en_passant_square);*/
        return None;
    }

    fn fix_move(&mut self, from: Square, to: Square, piece: _Index<Color>)
    {
        let n = match piece.get() { Color::White => 0, _ => 1 };  
        match &piece
        {
            &_Index::Pawn(_) => 
            {
                let p = self.pawns.get_mut(n).unwrap();
                let pawn = p.remove(&from).expect("fix pawn");
                p.insert(to.clone(), pawn);
            }
            &_Index::Knight(_) => 
            {
                let k = self.knights.get_mut(n).expect("fix knight 1");
                let knight =k.remove(&from).expect("fix knight 2");
                k.insert(to, knight);
            }
            &_Index::Bishop(_) =>
            {
                let b = self.bishops.get_mut(n).expect("fix bishop 1");
                let bishop = b.remove(&from).expect("fix bishop 2");
                b.insert(to, bishop);
            }
            &_Index::Rook(_) =>
            {
                let r = self.rooks.get_mut(n).expect("fix rook 1");
                let rook = r.remove(&from).expect("fix rook 2");
                r.insert(to, rook);
            }
            &_Index::Queen(_) =>
            {
                let q = self.queens.get_mut(n).expect("fix queen");
                q.0 = to;
            }

            &_Index::King(_) =>
            {
                let king = self.kings.get_mut(n).expect("fix king");
                king.0 = to;
            }

            _ => unreachable!()
        };
    }

    #[inline]
    fn get_n(&self, color: &Color) -> usize
    {
        match color { &Color::White => 0, _ => 1} 
    }
}
