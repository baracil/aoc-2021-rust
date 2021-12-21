use std::ops::Div;

const PLAYER1_POS: [usize; 10] = [6, 10, 2, 2, 10, 6, 10, 2, 2, 10];
const PLAYER2_POS: [usize; 10] = [5, 8, 9, 8, 5, 10, 3, 4, 3, 10];


pub struct Player {
    computed_pos: [usize; 10],
    score_per_cycle: usize,
    second_player:bool
}

impl Player {

    pub fn player1(starting_position: usize) -> Self {
        Player::player_with_pos(starting_position, &PLAYER1_POS,false)
    }

    pub fn player2(starting_position: usize) -> Self {
        Player::player_with_pos(starting_position, &PLAYER2_POS,true)
    }


    fn player_with_pos(starting_position: usize, pos: &[usize; 10], second_player:bool) -> Self {
        let computed_pos = pos.map(|i| modulate_position(i + starting_position));
        let score_per_cycle = computed_pos.iter().sum();
        Player { computed_pos, score_per_cycle, second_player }
    }

    pub fn score_at_turn(&self, turn: usize) -> usize {
        let mut score = 0;
        for i in 0..(turn % 10) {
            score += self.computed_pos[i];
        }

        score += (turn / 10) * self.score_per_cycle;

        score
    }

    pub fn winning_turn(&self) -> usize {
        let nb_cycles = 1000usize.div(self.score_per_cycle);

        let mut score_after_nb_cycles = nb_cycles * self.score_per_cycle;
        let mut nb_turns = nb_cycles * 10;
        let mut index = 0;

        while score_after_nb_cycles < 1000 {
            score_after_nb_cycles += self.computed_pos[index];
            nb_turns += 1;
            index += 1;
        };

        nb_turns
    }


    pub fn nb_rolls_to_win(&self) -> usize {
        let nb_turns = self.winning_turn();
        6*(nb_turns-1) + 3 + if self.second_player {3} else {0}
    }


}

fn modulate_position(pos: usize) -> usize {
    let modulo = pos % 10;
    if modulo == 0 { 10 } else { modulo }
}