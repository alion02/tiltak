use std::convert::TryFrom;
use std::fmt::{self, Write};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::{array, ops};

use board_game_traits::{Color, GameResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::position::bitboard::BitBoard;
use crate::position::color_trait::{BlackTr, ColorTr, WhiteTr};
use crate::position::utils::Direction::*;
use crate::position::Piece::{BlackCap, BlackFlat, BlackWall, WhiteCap, WhiteFlat, WhiteWall};
use crate::position::Role::{Cap, Flat, Wall};

/// A location on the board. Can be used to index a `Board`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Square(pub u8);

impl Square {
    pub const fn from_rank_file<const S: usize>(rank: u8, file: u8) -> Self {
        debug_assert!(rank < S as u8 && file < S as u8);
        Square(file * S as u8 + rank)
    }

    pub const fn rank<const S: usize>(self) -> u8 {
        if S == 6 {
            const RANKS: [[u8; 6]; 6] = {
                let mut ranks = [[0; 6]; 6];
                let mut file = 0;
                while file < 6 {
                    let mut rank = 0;
                    while rank < 6 {
                        ranks[file][rank] = rank as u8;
                        rank += 1;
                    }
                    file += 1;
                }

                ranks
            };

            debug_assert!((self.0 as usize) < 6 * 6);
            unsafe { (RANKS.as_ptr() as *const u8).add(self.0 as usize).read() }
        } else {
            self.0 % S as u8
        }
    }

    pub const fn file<const S: usize>(self) -> u8 {
        if S == 6 {
            const FILES: [[u8; 6]; 6] = {
                let mut files = [[0; 6]; 6];
                let mut file = 0;
                while file < 6 {
                    let mut rank = 0;
                    while rank < 6 {
                        files[file][rank] = file as u8;
                        rank += 1;
                    }
                    file += 1;
                }

                files
            };

            debug_assert!((self.0 as usize) < 6 * 6);
            unsafe { (FILES.as_ptr() as *const u8).add(self.0 as usize).read() }
        } else {
            self.0 / S as u8
        }
    }

    pub fn neighbours<const S: usize>(self) -> impl Iterator<Item = Square> {
        (if self.rank::<S>() == 0 && self.file::<S>() == 0 {
            &[(S as i8), 1][..]
        } else if self.rank::<S>() == 0 && self.file::<S>() == S as u8 - 1 {
            &[-(S as i8), 1][..]
        } else if self.rank::<S>() == S as u8 - 1 && self.file::<S>() == 0 {
            &[(S as i8), -1][..]
        } else if self.rank::<S>() == S as u8 - 1 && self.file::<S>() == S as u8 - 1 {
            &[-(S as i8), -1][..]
        } else if self.rank::<S>() == 0 {
            &[-(S as i8), (S as i8), 1][..]
        } else if self.rank::<S>() == S as u8 - 1 {
            &[-1, -(S as i8), (S as i8)][..]
        } else if self.file::<S>() == 0 {
            &[-1, (S as i8), 1][..]
        } else if self.file::<S>() == S as u8 - 1 {
            &[-1, -(S as i8), 1][..]
        } else {
            &[-1, -(S as i8), (S as i8), 1][..]
        })
        .iter()
        .map(move |sq| Square((sq + self.0 as i8) as u8))
    }

    pub fn directions<const S: usize>(self) -> impl Iterator<Item = Direction> {
        (if self.rank::<S>() == 0 && self.file::<S>() == 0 {
            &[East, South][..]
        } else if self.rank::<S>() == 0 && self.file::<S>() == S as u8 - 1 {
            &[West, South][..]
        } else if self.rank::<S>() == S as u8 - 1 && self.file::<S>() == 0 {
            &[East, North][..]
        } else if self.rank::<S>() == S as u8 - 1 && self.file::<S>() == S as u8 - 1 {
            &[West, North][..]
        } else if self.rank::<S>() == 0 {
            &[West, East, South][..]
        } else if self.rank::<S>() == S as u8 - 1 {
            &[North, West, East][..]
        } else if self.file::<S>() == 0 {
            &[North, East, South][..]
        } else if self.file::<S>() == S as u8 - 1 {
            &[North, West, South][..]
        } else {
            &[North, West, East, South][..]
        })
        .iter()
        .copied()
    }

    pub const fn go_direction<const S: usize>(self, direction: Direction) -> Option<Self> {
        self.jump_direction::<S>(direction, 1)
    }

    pub const fn jump_direction<const S: usize>(
        self,
        direction: Direction,
        len: u8,
    ) -> Option<Self> {
        let rank = self.rank::<S>();
        let file = self.file::<S>();
        match direction {
            North => {
                if let Some(new_rank) = rank.checked_sub(len) {
                    Some(Square::from_rank_file::<S>(new_rank, file))
                } else {
                    None
                }
            }
            West => {
                if file < len {
                    None
                } else {
                    Some(Square::from_rank_file::<S>(rank, file - len))
                }
            }
            East => {
                if file >= S as u8 - len {
                    None
                } else {
                    Some(Square::from_rank_file::<S>(rank, file + len))
                }
            }
            South => {
                if rank + len >= S as u8 {
                    None
                } else {
                    Some(Square::from_rank_file::<S>(rank + len, file))
                }
            }
        }
    }

    pub fn parse_square<const S: usize>(input: &str) -> Result<Square, pgn_traits::Error> {
        if input.len() != 2 {
            return Err(pgn_traits::Error::new_parse_error(format!(
                "Couldn't parse square \"{}\"",
                input
            )));
        }
        let mut chars = input.chars();
        let file = (chars.next().unwrap() as u8).overflowing_sub(b'a').0;
        let rank = (S as u8 + b'0')
            .overflowing_sub(chars.next().unwrap() as u8)
            .0;
        if file >= S as u8 || rank >= S as u8 {
            Err(pgn_traits::Error::new_parse_error(format!(
                "Couldn't parse square \"{}\" at size {}",
                input, S
            )))
        } else {
            Ok(Square::from_rank_file::<S>(rank, file))
        }
    }

    pub fn to_string<const S: usize>(self) -> String {
        let mut string = String::new();
        write!(string, "{}", (self.file::<S>() + b'a') as char).unwrap();
        write!(string, "{}", S as u8 - self.rank::<S>()).unwrap();
        string
    }
}

/// Iterates over all board squares.
pub fn squares_iterator<const S: usize>() -> impl Iterator<Item = Square> {
    (0..(S * S)).map(|i| Square(i as u8))
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Komi {
    half_komi: i8,
}

impl Komi {
    pub fn from_half_komi(half_komi: i8) -> Option<Self> {
        if (-10..=10).contains(&half_komi) {
            Some(Komi { half_komi })
        } else {
            None
        }
    }

    pub fn half_komi(self) -> i8 {
        self.half_komi
    }

    pub fn game_result_with_flatcounts(self, white_flats: i8, black_flats: i8) -> GameResult {
        match (2 * (white_flats - black_flats) - self.half_komi).signum() {
            -1 => GameResult::BlackWin,
            0 => GameResult::Draw,
            1 => GameResult::WhiteWin,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<f64> for Komi {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        // Match against a list of float literals to convert,
        // to avoid any float math shenanigans
        if let Some((_, half_komi)) = [
            -5.0, -4.5, -4.0, -3.5, -3.0, -2.5, -2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0,
            2.5, 3.0, 3.5, 4.0, 4.5, 5.0,
        ]
        .iter()
        .zip(-10..=10)
        .find(|(komi, _)| **komi == value)
        {
            Ok(Komi { half_komi })
        } else {
            Err(format!("Invalid komi {}", value))
        }
    }
}

impl TryFrom<f32> for Komi {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

impl FromStr for Komi {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        f64::from_str(value)
            .map_err(|err| err.to_string())
            .and_then(Self::try_from)
    }
}

impl From<Komi> for f64 {
    fn from(komi: Komi) -> Self {
        komi.half_komi as f64 / 2.0
    }
}

impl From<Komi> for f32 {
    fn from(komi: Komi) -> Self {
        komi.half_komi as f32 / 2.0
    }
}

impl fmt::Display for Komi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f64_komi: f64 = (*self).into();
        f64_komi.fmt(f)
    }
}

/// One of the 3 piece roles in Tak. The same as piece, but without different variants for each color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Role {
    Flat,
    Wall,
    Cap,
}

impl Role {
    pub fn disc(self) -> usize {
        self as u16 as usize
    }
}

/// One of the 6 game pieces in Tak. Each piece has one variant for each color.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Piece {
    WhiteFlat = 0,
    BlackFlat = 1,
    WhiteWall = 2,
    BlackWall = 3,
    WhiteCap = 4,
    BlackCap = 5,
}

impl Piece {
    pub fn from_role_color(role: Role, color: Color) -> Self {
        match (role, color) {
            (Flat, Color::White) => WhiteFlat,
            (Wall, Color::White) => WhiteWall,
            (Cap, Color::White) => WhiteCap,
            (Flat, Color::Black) => BlackFlat,
            (Wall, Color::Black) => BlackWall,
            (Cap, Color::Black) => BlackCap,
        }
    }

    pub fn role(self) -> Role {
        match self {
            WhiteFlat | BlackFlat => Flat,
            WhiteWall | BlackWall => Wall,
            WhiteCap | BlackCap => Cap,
        }
    }

    pub fn color(self) -> Color {
        match self {
            WhiteFlat | WhiteWall | WhiteCap => Color::White,
            BlackFlat | BlackWall | BlackCap => Color::Black,
        }
    }

    pub fn is_road_piece(self) -> bool {
        WhiteTr::is_road_stone(self) || BlackTr::is_road_stone(self)
    }

    pub fn flip_color(self) -> Self {
        match self {
            WhiteFlat => BlackFlat,
            BlackFlat => WhiteFlat,
            WhiteWall => BlackWall,
            BlackWall => WhiteWall,
            WhiteCap => BlackCap,
            BlackCap => WhiteCap,
        }
    }
}

impl ops::Not for Piece {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            WhiteFlat => BlackFlat,
            BlackFlat => WhiteFlat,
            WhiteWall => BlackWall,
            BlackWall => WhiteWall,
            WhiteCap => BlackCap,
            BlackCap => WhiteCap,
        }
    }
}

/// The contents of a square on the board, consisting of zero or more pieces
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stack {
    pub(crate) top_stone: Option<Piece>,
    pub(crate) bitboard: BitBoard,
    pub(crate) height: u8,
}

impl Stack {
    /// Get a piece by index. 0 is the bottom of the stack
    pub fn get(&self, i: u8) -> Option<Piece> {
        if i >= self.height {
            None
        } else if i == self.height - 1 {
            self.top_stone
        } else if self.bitboard.get(i) {
            Some(WhiteFlat)
        } else {
            Some(BlackFlat)
        }
    }

    pub fn top_stone(&self) -> Option<Piece> {
        self.top_stone
    }

    /// Push a new piece to the top of the stack
    ///
    /// Any piece already on the stack will be flattened, including capstones
    pub fn push(&mut self, piece: Piece) {
        if self.height > 0 && self.top_stone.unwrap().color() == Color::White {
            self.bitboard = self.bitboard.set(self.height - 1);
        }
        self.top_stone = Some(piece);
        self.height += 1;
    }

    /// Remove the top piece from the stack, a
    ///
    /// Will not un-flatten a previously flattened stone
    pub fn pop(&mut self) -> Option<Piece> {
        debug_assert_ne!(self.height, 0);
        let old_piece = self.top_stone;
        if self.height > 1 {
            let piece = if self.bitboard.get(self.height - 2) {
                Piece::WhiteFlat
            } else {
                Piece::BlackFlat
            };
            self.bitboard = self.bitboard.clear(self.height - 2);
            self.top_stone = Some(piece);
        } else {
            self.top_stone = None;
        }
        self.height -= 1;
        old_piece
    }

    pub fn replace_top(&mut self, piece: Piece) -> Option<Piece> {
        self.top_stone.replace(piece)
    }

    pub fn remove(&mut self, i: u8) -> Piece {
        if i == self.height - 1 {
            self.pop().expect("Tried to remove from empty stack")
        } else {
            let piece = if self.bitboard.get(i) {
                Piece::WhiteFlat
            } else {
                Piece::BlackFlat
            };
            let pieces_below = self.bitboard & BitBoard::lower_n_bits(i);
            let pieces_above = self.bitboard & !BitBoard::lower_n_bits(i + 1);
            self.bitboard = pieces_below
                | BitBoard {
                    board: pieces_above.board >> 1,
                };
            self.height -= 1;
            piece
        }
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0
    }

    pub fn len(&self) -> u8 {
        self.height
    }
}

/// An iterator over the pieces in a stack, from the bottom up
pub struct StackIterator {
    stack: Stack,
}

impl Iterator for StackIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

impl IntoIterator for Stack {
    type Item = Piece;
    type IntoIter = StackIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { stack: self }
    }
}

/// One of the four cardinal directions on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    pub(crate) fn reverse(self) -> Direction {
        match self {
            North => South,
            West => East,
            East => West,
            South => North,
        }
    }

    pub(crate) fn orthogonal_directions(self) -> [Direction; 2] {
        match self {
            North | South => [West, East],
            West | East => [North, South],
        }
    }

    pub(crate) fn parse(ch: char) -> Option<Self> {
        match ch {
            '+' => Some(North),
            '<' => Some(West),
            '>' => Some(East),
            '-' => Some(South),
            _ => None,
        }
    }
}

/// One or more `Movement`s, storing how many pieces are dropped off at each step
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackMovement {
    // The first 4 bits is the number of squares moved
    // The remaining 28 bits are the number of pieces taken, 4 bits per number
    data: u8,
}

impl StackMovement {
    pub fn new() -> Self {
        StackMovement { data: 0 }
    }

    pub fn get_first<const S: usize>(&self) -> Movement {
        Movement {
            pieces_to_take: 8 - self.data.leading_zeros() as u8,
        }
    }

    pub fn push<const S: usize>(&mut self, movement: Movement, pieces_held: u8) {
        debug_assert!(pieces_held > 0);
        debug_assert!(
            self.data == 0 || pieces_held > movement.pieces_to_take,
            "data {:b}, {} pieces held, taking {}",
            self.data,
            pieces_held,
            movement.pieces_to_take
        );

        let pieces_to_drop = pieces_held - movement.pieces_to_take;

        if self.data != 0 {
            self.data <<= pieces_to_drop - 1;
        }
        if movement.pieces_to_take > 0 {
            self.data <<= 1;
            self.data |= 1;
        }
    }

    pub fn len(self) -> usize {
        self.data.count_ones() as usize
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn from_movements<const S: usize, I: IntoIterator<Item = Movement>>(iter: I) -> Self {
        let mut pieces_held = S as u8;
        let mut result = StackMovement::new();
        for movement in iter {
            // println!("Holding {}, taking {}", pieces_held, movement.pieces_to_take);
            result.push::<S>(movement, pieces_held);
            pieces_held = movement.pieces_to_take;
        }
        result
    }

    #[allow(clippy::should_implement_trait)]
    pub fn into_iter<const S: usize>(self) -> impl Iterator<Item = Movement> {
        StackMovementIterator { data: self.data }
    }
}

pub struct StackMovementIterator {
    data: u8,
}

impl Iterator for StackMovementIterator {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data == 0 {
            None
        } else {
            let pieces_to_take = 8 - self.data.leading_zeros() as u8;
            self.data &= !(1 << (pieces_to_take - 1));
            Some(Movement { pieces_to_take })
        }
    }
}

/// Moving a stack of pieces consists of one or more `Movement`s
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Movement {
    pub pieces_to_take: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractBoard<T, const S: usize> {
    pub(crate) raw: [[T; S]; S],
}

impl<T: Copy, const S: usize> AbstractBoard<T, S> {
    pub const fn new_with_value(value: T) -> Self {
        AbstractBoard {
            raw: [[value; S]; S],
        }
    }
}

impl<T: Copy, const S: usize> AbstractBoard<T, S> {
    pub fn new_from_fn<F>(mut f: F) -> Self
    where
        F: FnMut() -> T,
    {
        AbstractBoard {
            raw: array::from_fn(|_| array::from_fn(|_| f())),
        }
    }
}

pub(crate) const fn generate_neighbor_table<const S: usize>() -> AbstractBoard<BitBoard, S> {
    let mut table = AbstractBoard::new_with_value(BitBoard::empty());
    let mut rank = 0;
    while rank < S {
        let mut file = 0;
        while file < S {
            let square = Square::from_rank_file::<S>(rank as u8, file as u8);
            table.raw[file][rank] = BitBoard::neighbors::<S>(square);
            file += 1;
        }
        rank += 1;
    }
    table
}

const NEIGHBOR_TABLE_3S: AbstractBoard<BitBoard, 3> = generate_neighbor_table::<3>();
const NEIGHBOR_TABLE_4S: AbstractBoard<BitBoard, 4> = generate_neighbor_table::<4>();
const NEIGHBOR_TABLE_5S: AbstractBoard<BitBoard, 5> = generate_neighbor_table::<5>();
const NEIGHBOR_TABLE_6S: AbstractBoard<BitBoard, 6> = generate_neighbor_table::<6>();
const NEIGHBOR_TABLE_7S: AbstractBoard<BitBoard, 7> = generate_neighbor_table::<7>();
const NEIGHBOR_TABLE_8S: AbstractBoard<BitBoard, 8> = generate_neighbor_table::<8>();

pub(crate) fn lookup_neighbor_table<const S: usize>(square: Square) -> BitBoard {
    match S {
        3 => NEIGHBOR_TABLE_3S[square],
        4 => NEIGHBOR_TABLE_4S[square],
        5 => NEIGHBOR_TABLE_5S[square],
        6 => NEIGHBOR_TABLE_6S[square],
        7 => NEIGHBOR_TABLE_7S[square],
        8 => NEIGHBOR_TABLE_8S[square],
        _ => unimplemented!("Unsupported size {}", S),
    }
}

impl<T: Default + Copy, const S: usize> Default for AbstractBoard<T, S> {
    fn default() -> Self {
        AbstractBoard {
            raw: [[T::default(); S]; S],
        }
    }
}

impl<T, const S: usize> Index<Square> for AbstractBoard<T, S> {
    type Output = T;
    #[allow(clippy::needless_lifetimes)]
    fn index<'a>(&'a self, square: Square) -> &'a Self::Output {
        debug_assert!((square.0 as usize) < S * S);
        // Compared to the safe code, this is roughly a 10% speedup of the entire engine
        unsafe {
            (self.raw.as_ptr() as *const T)
                .offset(square.0 as isize)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<T, const S: usize> IndexMut<Square> for AbstractBoard<T, S> {
    #[allow(clippy::needless_lifetimes)]
    fn index_mut<'a>(&'a mut self, square: Square) -> &'a mut Self::Output {
        debug_assert!((square.0 as usize) < S * S);
        // Compared to the safe code, this is roughly a 10% speedup of the entire engine
        unsafe {
            (self.raw.as_mut_ptr() as *mut T)
                .offset(square.0 as isize)
                .as_mut()
                .unwrap_unchecked()
        }
    }
}
