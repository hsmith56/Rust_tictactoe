use std::{ fmt, io, str };

#[derive(Clone, Debug)]
struct Player {
    name: String,
    piece: char,
    repr: u8
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Piece: {}, repr: {}", self.name, self.piece, self.repr)
    }
}

// Wanted to do this so that i could be able to make a default player, more precisely the
// player that represents an empty piece on the board
impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from("empty"),
            piece: ' ',
            repr: 0
        }
    }
}

impl Player {
    fn new(name: String, piece: char, repr: u8) -> Self {
        Self {
            name, piece, repr,
        }
    }
}

#[derive(Debug)]
struct Game {
    winner: Player,
    players: Vec<Player>,
    board: Vec<u8>,
}

impl Game {
    // TODO: Randomly assign pieces
    fn new(p1: &str, p2: &str) -> Self {
        let default_player: Player = Default::default();
        Self {
            players: vec![
                default_player.clone(), 
                Player::new(String::from(p1), 'X', 1), 
                Player::new(String::from(p2), 'O', 2)],
            board: vec![0; 9],
            winner: default_player
        }
    }

    fn set_winner(&mut self, repr: u8) {
        self.winner = self.players.iter().find(|p| p.repr == repr).map(|f| f).unwrap().clone();
    }

    fn is_game_over(&mut self) -> bool {
        // TODO: What better ways are there to do this?
        // TODO: Is the way that i am setting the winner a smart way? It seems like it could be easier but maybe that's python brain talking

        for i in 0..=2 {
            if self.board[i] == 0 {
                continue
            }
            // Vertical win (columns are all the same)
            if &self.board[i] == &self.board[i+3] && &self.board[i+3] == &self.board[i+6] {
                self.set_winner(self.board[i]);
                return true
            }
            // Horizontal win (rows are all the same)
            if *&self.board[i*3] != 0 && &self.board[i*3] == &self.board[i*3+1] && &self.board[i*3+1] == &self.board[i*3+2] {
                println!("here {}", &self.board[i]);
                self.set_winner(self.board[i]);
                return true
            }
        }
        // diagional win top left to bottom right
        if *&self.board[0] != 0 && &self.board[0] == &self.board[4] && &self.board[4] == &self.board[8] {
            self.set_winner(self.board[0]);
            return true
        }
        // diagional win top right to bottom left
        if *&self.board[2] != 0 && &self.board[2] == &self.board[4] && &self.board[4] == &self.board[6] {
            self.set_winner(self.board[2]);
            return true
        }

        // return true if all board positions are not empty (0), otherwise return false
        !self.board.contains(&0)
    } 

    fn _is_a_valid_move(&self, location: &usize) -> bool {
        self.board[*location] == 0
    }

    fn make_move(&mut self, player_repr: u8, location: usize) -> Option<bool> {
        if self._is_a_valid_move(&location) == false {
            return None
        }
        else {
            self.board[location] = player_repr;
            return Some(true)
        }
    }
    
    // TODO: Do actual formatting, maybe replace the blank spaces with numbers so you know 
    // what moves are available
    fn display_board(&self) {
        for i in (0..=6).step_by(3) {
            let rows = &self.board[i..=i+2].iter()
                .map(|&x| self.players.iter()
                    .find(|p| p.repr == x)
                    .map(|f| f.piece)
                    .unwrap())
                .collect::<Vec<char>>();
            println!("{:?}", rows);
            // println!("{:?}",&self.board[i..=i+2]);
        }
    }
}

fn game_loop(mut game: Game) -> Game {
    let mut who_moves_next: u8 = 1;
    
    // step 3: alternate players making moves until game is over 
    while game.is_game_over() == false {
        game.display_board();
        println!("Player {}, enter your move", who_moves_next);
        let mut move_loc: String = String::new();
        io::stdin().read_line(&mut move_loc).expect("Failed to read in input");
        let move_location = move_loc.trim().parse::<usize>();
        match move_location {
            Ok(num) => {
                match game.make_move(who_moves_next, num) {
                    // if the move is valid, alternate to other player. Otherwise, try again.
                    // probably a few ways to do this
                  Some(_) => match who_moves_next {
                        1 => who_moves_next = 2,
                        2 => who_moves_next = 1,
                        _ => who_moves_next = 0
                    },
                  None => println!("Invalid move, please try again")
                }
            },
            Err(e) => println!("Invalid input provided, please try again. {e}")
        }
    }
    game
}
fn main() {
    // Step 1: get input for player 1 and player 2
    let mut player_1: String = String::new();
    let mut player_2: String = String::new();
    println!("Please enter player 1's name");
    io::stdin().read_line(&mut player_1).expect("Failed to read in input");
    println!("Please enter player 2's name");
    io::stdin().read_line(&mut player_2).expect("Failed to read in input");

    let player_1 = player_1.trim();
    let player_2 = player_2.trim();
    let mut game: Game = Game::new(player_1, player_2);
    // Step 2: start game loop
    game = game_loop(game);
    game.display_board();

    // Step 4: Announce results of the game
    match &game.winner {
        Player {repr : 1 , ..} => println!("Player 1 wins, congratulations {}!", &game.winner.name),
        Player {repr : 2, ..} => println!("Player 2 wins, congratulations {}!", &game.winner.name),
        Player {..} => println!("No one wins, it's a tie!"),
    };

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn player_created_successfully() {
        let name: String = String::from("Bing Bong");
        let me: Player = Player::new(name, 'X', 1);
        assert_eq!(me.name, "Harrison");
        assert_eq!(me.piece, 'X');
        assert_eq!(me.repr, 1);
    }

    #[test]
    fn game_board_initialized() {
        let game: Game = Game::new("Me", "my dog");
        assert_eq!(game.board.len(), 9);
        assert_eq!(game.winner.repr, 0);
    }

    #[test]
    fn test_win_conditions() {
        let mut game: Game = Game::new("Me", "my dog");
        game.board[0] = 1;
        game.board[4] = 1;
        game.board[8] = 1;
        game.is_game_over();
        assert_eq!(game.winner.repr, 1);
    }
}