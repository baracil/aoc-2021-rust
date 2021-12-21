use std::collections::VecDeque;

const ROLL: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];
const OCCURRENCES: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];


pub struct WonGames {
    player1: usize,
    player2: usize,
}

impl WonGames {
    pub fn player1(&self) -> usize {
        self.player1
    }
    pub fn player2(&self) -> usize {
        self.player2
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    position: usize,
    score: usize,
}

#[derive(Copy, Clone)]
pub struct Turn {
    player1: Player,
    player2: Player,
    roll_index:usize,
    nb_multiverses:usize,
    player1_turn:bool,
}

impl Turn {
    fn perform_turn_make_a_win(&mut self, nb_win_player1:&mut usize, nb_win_player2: &mut usize) -> bool {
        self.nb_multiverses *= OCCURRENCES[self.roll_index];
        if self.player1_turn {
            self.player1.move_by(ROLL[self.roll_index]);
            if self.player1.won() {
                *nb_win_player1 += self.nb_multiverses;
            }
        } else {
            self.player2.move_by(ROLL[self.roll_index]);
            if self.player2.won() {
                *nb_win_player2 += self.nb_multiverses;
            }
        }

        self.player1_turn = !self.player1_turn;
        self.player1.won() || self.player2.won()
    }

    fn with_roll_index(&self, roll_index:usize) -> Self {
        let mut result = *self;
        result.roll_index = roll_index;
        result
    }
}

impl Player {
    pub fn create(player_position: usize) -> Self {
        Player { position: player_position, score: 0 }
    }

    pub fn move_by(&mut self, roll: usize) {
        self.position = modulate_position(self.position + roll);
        self.score += self.position;
    }

    pub fn won(&self) -> bool {
        self.score >= 21
    }

}


pub fn count_won_games(player1_start: usize, player2_start: usize) -> WonGames {

    let player1 = Player::create(player1_start);
    let player2 = Player::create(player2_start);

    let mut nb_win_player1 = 0_usize;
    let mut nb_win_player2 = 0_usize;

    let mut stack = VecDeque::<Turn>::new();

    (0..ROLL.len()).for_each(|i| stack.push_back(Turn {player1,player2, roll_index:i, nb_multiverses:1, player1_turn:true}));

    while let Some(mut turn) = stack.pop_front() {
        if !turn.perform_turn_make_a_win(&mut nb_win_player1,&mut nb_win_player2) {
            (0..ROLL.len()).for_each(|i| stack.push_back(turn.with_roll_index(i)));
        }
    };

    WonGames{ player1:nb_win_player1, player2:nb_win_player2}

}


fn modulate_position(pos: usize) -> usize {
    let modulo = pos % 10;
    if modulo == 0 { 10 } else { modulo }
}