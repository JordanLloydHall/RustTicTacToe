use std::io;
use std::cmp;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

// Use 3 or 4. 5 is not recommended all that much.
const BOARD_SIZE: usize = 4;

// Game struct uses a stack approach to 
#[derive(Debug, Clone)]
struct Game {
    size: usize,
    state: Vec<i8>,
    transpose_table: HashMap<Vec<i8>, i8>
}


impl Game {

    fn new() -> Game {
        return Game{
            state: vec![0; BOARD_SIZE * BOARD_SIZE],
            size: BOARD_SIZE,
            transpose_table: HashMap::new()
        }
    }

    fn get_board_state(&self) -> Vec<i8> {
        return self.state.clone();
    }

    fn execute_move(&mut self, pos: usize, player: i8) {
        self.state[pos] = player;
    }

    fn get_winner(&self) -> Option<i8> {

        let curr_state = self.get_board_state();

        for row in 0..self.size {
            let mut has_won = true;
            let player = curr_state[row * self.size];
            for col in 0..self.size {
                has_won &= curr_state[row * self.size + col] == player;
            }
            if player != 0 && has_won {
                return Some(player);
            }
        }

        for col in 0..self.size {
            let mut has_won = true;
            let player = curr_state[col];
            for row in 0..self.size {
                has_won &= curr_state[row * self.size + col] == player;
            }
            if player != 0 &&  has_won {
                return Some(player);
            }
        }

        let mut has_won = true;
        let player = curr_state[0];
        for diag in 0..self.size {
            has_won &= curr_state[diag * self.size + diag] == player;
        }
        if player != 0 && has_won {
            return Some(player);
        }

        let mut has_won = true;
        let player = curr_state[self.size - 1];
        for diag in 0..self.size {
            has_won &= curr_state[diag * self.size + self.size - 1 - diag] == player;
        }
        if player != 0 && has_won {
            return Some(player);
        }

        let mut is_tie = true;
        for place in curr_state.iter() {
            is_tie &= *place != 0;
        }
        if is_tie {
            return Some(0);
        }

        None
    }

    fn is_legal_move(&self, pos: usize) -> bool {
        let curr_state = self.get_board_state();

        match curr_state[pos] {
            0 => return true,
            _ => return false
        }
    }

    fn get_legal_moves(&self) -> Vec<usize> {
        let curr_state = self.get_board_state();
        return (0..self.size * self.size).filter(|n| curr_state[*n as usize] == 0).collect();
    }

    fn min_max(&mut self, player: i8, maximizing: bool, mut alpha: i8, mut beta: i8, depth: i8) -> i8 {

        if self.get_winner().is_some() {
            return self.score_game(player, maximizing);
        }
        if depth == 0 {
            return 0;
        }

        if maximizing {
            let moves = self.get_legal_moves();
            let mut val = -1;
            for mov in moves {
                let old_state = self.get_board_state();
                self.execute_move(mov, player);

                let curr_state = self.get_board_state();
                if self.transpose_table.contains_key(&curr_state) {
                    val = cmp::max(val, *self.transpose_table.get(&curr_state).expect("Transpostition table does not have state as key"));
                } else {
                    let minimaxed_score = self.min_max(if player == 1 { 2 } else { 1 }, false, alpha, beta, depth - 1);
                    val = cmp::max(val, minimaxed_score);
                    self.transpose_table.insert(curr_state, minimaxed_score);
                }
                self.state = old_state;

                if val >= beta {
                    break;
                }
                alpha = cmp::max(alpha, val);
                
            }

            return val;

        } else {
            let moves = self.get_legal_moves();
            let mut val = 1;
            for mov in moves {
                let old_state = self.get_board_state();
                self.execute_move(mov, player);
                let curr_state = self.get_board_state();
                
                if self.transpose_table.contains_key(&curr_state) {
                    val = cmp::min(val, *self.transpose_table.get(&curr_state).expect("Transpostition table does not have state as key"));
                } else {
                    let minimaxed_score = self.min_max(if player == 1 { 2 } else { 1 }, true, alpha, beta, depth - 1);
                    val = cmp::min(val, minimaxed_score);
                    self.transpose_table.insert(curr_state, minimaxed_score);
                }
                self.state = old_state;

                if val <= alpha {
                    break;
                }
                beta = cmp::min(beta, val);
                
            }

            return val;
        }
    }

    fn score_game(&self, player: i8, maximizing: bool) -> i8 {
        let winner = self.get_winner().expect("Tried to score a match where nobody won");

        if winner == 0 {
            return 0;
        } else if winner == player {
            if maximizing {
                return 1;
            } else {
                return -1
            }
            
        } else {
            if maximizing {
                return -1;
            } else {
                return 1
            }
        }
    }

    fn get_best_move(&mut self, scores: Vec<i8>) -> usize {
        return *self.get_legal_moves()
        .iter()
        .max_by_key(|n| {
            return scores[**n as usize];
        })
        .unwrap();
    } 

    fn score_moves(&mut self, player: i8) -> Vec<i8> {
        return (0..self.size * self.size)
        .into_par_iter()
        .map(|n| {
            if !self.is_legal_move(n) {
                return i8::MIN;
            }
            let mut new_board = self.clone();
            
            new_board.execute_move(n, player);
            let score = new_board.min_max(if player == 1 { 2 } else { 1 }, false, i8::MIN, i8::MAX, i8::MAX);
            return score;
        })
        .collect();
    }


    fn play_game(&mut self) {
        let mut player = 1;
        while self.get_winner().is_none() {
            self.pretty_print();

            if player == 2 {
                let mut turn_move = String::new();

                io::stdin()
                    .read_line(&mut turn_move)
                    .expect("Failed to read line");

                let pos: usize = turn_move.trim().parse().expect("Not a number");

                if self.is_legal_move(pos) {
                    self.execute_move(pos, player);
                    player = if player == 1 { 2 } else { 1 };
                } else {
                    println!("Not a legal move mate");
                }
            } else {
                let now = Instant::now();
                let scores = self.score_moves(player);
                println!("Move Scores: {:?}\nTime elapsed: {}", scores, now.elapsed().as_millis());
                let best_move = self.get_best_move(scores);
                self.execute_move(best_move, player);

                player = if player == 1 { 2 } else { 1 };
            }
            
        }

        println!("{:?}", self.get_winner());
    }

    fn pretty_print(&self) {
        let curr_state = self.get_board_state();

        for row in 0..self.size {
            for col in 0..self.size {
                if curr_state[row*self.size + col] != 0 {
                    print!("{}", curr_state[row*self.size + col]);
                } else {
                    print!(" ");
                }
                if col != self.size-1 {
                    print!("|");
                }
                
            }
            if row != self.size-1 {
                print!("\n-----\n");
            } else {
                print!("\n");
            }
            
        }
        

    }
}



fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    
    game.play_game();
}

