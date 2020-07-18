use crate::board::Board;
use crate::playtak;
use board_game_traits::board::Board as BoardTrait;
use pgn_traits::pgn::PgnBoard;

#[test]
fn parse_place_move_test() {
    let move_strings = [
        ("P A1", "a1"),
        ("P A1 W", "Sa1"),
        ("P A1 C", "Ca1"),
        ("P D3 W", "Sd3"),
    ];

    for (playtak_move_string, san_move_string) in move_strings.iter() {
        assert_eq!(
            playtak::parse_move(playtak_move_string).to_string(),
            *san_move_string
        );
    }
}

#[test]
fn parse_move_move_test() {
    let move_strings = [("M A1 C1 1 2", "3a1>12"), ("M C2 C3 1", "c2+")];

    for (playtak_move_string, san_move_string) in move_strings.iter() {
        assert_eq!(
            playtak::parse_move(playtak_move_string).to_string(),
            *san_move_string
        );
    }
}

#[test]
fn write_place_move_test() {
    let move_strings = [
        ("P A1", "a1"),
        ("P A1 W", "Sa1"),
        ("P A1 C", "Ca1"),
        ("P D3 W", "Sd3"),
    ];

    for (playtak_move_string, san_move_string) in move_strings.iter() {
        let board = Board::start_board();
        let mut sink = String::new();
        playtak::write_move(board.move_from_san(san_move_string).unwrap(), &mut sink);
        assert_eq!(sink, *playtak_move_string);
    }
}

#[test]
fn write_move_move_test() {
    let move_strings = [("M A1 C1 1 2", "3a1>12"), ("M C2 C3 1", "c2+")];

    for (playtak_move_string, san_move_string) in move_strings.iter() {
        let board = Board::start_board();
        let mut sink = String::new();
        playtak::write_move(board.move_from_san(san_move_string).unwrap(), &mut sink);
        assert_eq!(sink, *playtak_move_string);
    }
}