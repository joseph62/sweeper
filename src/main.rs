use std::io;
use std::fmt;

const BOARD_X: usize = 5;
const BOARD_Y: usize = 5;

fn main() {

    let mut board = new_board();
    let mut score = 0;
    let mut player_position = (0, 0);
    place_piece_on_board(&mut board, Piece::Player, player_position);

    loop { 

        println!("");
        println!("Score: {}", score);
        println!("Position: ({}, {})", player_position.0, player_position.1);
        println!("");
        display_board(&board);
        println!("");
        
        let calculate_new_position = match next_player_game_action() {
            PlayerGameAction::Up => |p| calculate_new_position(p, (0, -1)),
            PlayerGameAction::Down => |p| calculate_new_position(p, (0, 1)),
            PlayerGameAction::Left => |p| calculate_new_position(p, (-1, 0)),
            PlayerGameAction::Right => |p| calculate_new_position(p, (1, 0)),
            PlayerGameAction::Exit => {
                println!("Exiting game...");
                break;
            },        
        };

        let next_position = calculate_new_position(player_position);
        if next_position != player_position {
            match move_piece_on_board(&mut board, player_position, next_position) {
                Piece::Rubbish(n) => score += n,
                _ => {},
            };
            player_position = next_position;
        }
    }
} 

enum PlayerGameAction {
    Exit,
    Up,
    Down,
    Left,
    Right
}

fn next_player_game_action() -> PlayerGameAction {
    let mut action = String::new();

    println!("\nEnter the your next game action (? for help): ");

    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read next player game action");

    println!("");
    
    return match action.trim() {
        "exit" => PlayerGameAction::Exit,
        "up" => PlayerGameAction::Up,
        "down" => PlayerGameAction::Down,
        "left" => PlayerGameAction::Left,
        "right" => PlayerGameAction::Right,
        "?" => {
            input_help();
            next_player_game_action()
        },
        _ => {
            println!("Unrecognized input. Enter '?' for help.");
            next_player_game_action()
        },
    }
}

fn input_help() {
    println!("You may enter one of the following actions:");
    println!("\tup - Move up");
    println!("\tdown - Move down");
    println!("\tleft - Move left");
    println!("\tright - Move right");
    println!("\texit - Exit the game");
    println!("\t? - Show this help message");
    println!("");
}

#[derive(Clone, Copy)]
enum Piece {
    Player,
    Empty,
    Rubbish(u32)
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Player => write!(f, "{}", "x"),
            Piece::Empty => write!(f, "{}", "0"),
            Piece::Rubbish(n) => write!(f, "{}", n),
        }
    }
}

fn calculate_new_position(current: (usize, usize), by: (i8, i8)) -> (usize, usize) {
    let (x, y) = current;
    let (by_x, by_y) = by;
    let x = inbetween_range(x as i8 + by_x, 0, BOARD_X as i8);
    let y = inbetween_range(y as i8 + by_y, 0, BOARD_Y as i8);
    (x as usize, y as usize)
}

fn inbetween_range(value: i8, low: i8, high: i8) -> i8{
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

fn new_board() -> [[Piece; BOARD_X]; BOARD_Y] {
    return [
        [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
        [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Rubbish(1)],
        [Piece::Rubbish(2), Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
        [Piece::Empty, Piece::Empty, Piece::Rubbish(3), Piece::Empty, Piece::Empty],
        [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty],
    ]
}

fn place_piece_on_board(board: &mut [[Piece; BOARD_X]; BOARD_Y], piece: Piece, position: (usize, usize)) -> Piece {
    let (x, y) = position;
    let x = x % BOARD_X;
    let y = y % BOARD_Y;
    let last = board[y][x];
    board[y][x] = piece;
    last
}

fn move_piece_on_board(board: &mut [[Piece; 5]; 5], from: (usize, usize), to: (usize, usize)) -> Piece {
    let piece = place_piece_on_board(board, Piece::Empty, from);
    place_piece_on_board(board, piece, to)
}

fn display_board(board: &[[Piece; 5]]) {
    for (i, row) in board.iter().enumerate() {
        for col in row.iter() {
            print!("{} ", col);
        }
        if i != board.len() - 1 {
            println!("");
        }
    } 
}