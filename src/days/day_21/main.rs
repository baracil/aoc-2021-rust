use crate::{Part, try_parse_input};
use crate::days::day_21::multi_verse_game::count_won_games;
use crate::days::day_21::score_computer::Player;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day21_launch(part: Part) -> AOCResult<String> {
    let starting_positions = parse_input(false)?;
    match part {
        Part::Part1 => part1(&starting_positions),
        Part::Part2 => part2(&starting_positions)
    }
}

fn part1(start_position:&(usize,usize)) -> AOCResult<String> {
    let player1 = Player::player1(start_position.0);
    let player2 = Player::player2(start_position.1);

    let player1_nb_rolls_to_win = player1.nb_rolls_to_win();
    let player2_nb_rolls_to_win = player2.nb_rolls_to_win();

    let result =
    if player1_nb_rolls_to_win<player2_nb_rolls_to_win {
        player2.score_at_turn(player1.winning_turn()-1)*player1_nb_rolls_to_win
    } else {
        player1.score_at_turn(player2.winning_turn())*player2_nb_rolls_to_win
    };

    Ok(result.to_string())
}

fn part2(starting_position:&(usize,usize)) -> AOCResult<String> {
    let games = count_won_games(starting_position.0, starting_position.1);
    Ok(games.player1().max(games.player2()).to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<(usize,usize)> {
    let positions:Vec<usize> = Problem::factory(for_test)(21)
        .read_input_as_mapped_lines(|l| parse_starting_position(l).unwrap())?;


    Ok((positions[0], positions[1]))
}

pub fn parse_starting_position(line:&str) -> AOCResult<usize> {
    try_parse_input!(line.split_once(":")
        .ok_or_else(|| "Cannot parse starting position".to_string())
        .map(|(_,val)| val.trim_start()).unwrap(),usize)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_21::main::{parse_input, part1, part2};
    use crate::days::day_21::score_computer::Player;

    #[test]
    fn day21_do_100_turn() {
        let player1 = Player::player1(4);
        let player2 = Player::player2(8);
        for i in 120..200 {
            println!("{} {} {}", i, player1.score_at_turn(i), player2.score_at_turn(i));
        }
        println!("{}", player1.winning_turn());
    }

    #[test]
    fn day21_score_player_at_first_turn()  {
        let player = Player::player1(4);
        assert_eq!(player.score_at_turn(1),10);
    }
    #[test]
    fn day21_score_player_at_second_turn()  {
        let player = Player::player1(4);
        assert_eq!(player.score_at_turn(2),14);
    }
    #[test]
    fn day21_score_player_at_third_turn()  {
        let player = Player::player1(4);
        assert_eq!(player.score_at_turn(3),20);
    }
    #[test]
    fn day21_score_player_at_fourth_turn()  {
        let player = Player::player1(4);
        assert_eq!(player.score_at_turn(4),26);
    }

    #[test]
    fn day21_score_player1_winning_turn()  {
        let player = Player::player1(4);
        assert_eq!(player.nb_rolls_to_win(),993);
    }

    #[test]
    fn day21_part1_test()  {
        let start_position = parse_input(true).unwrap();
        let result = part1(&start_position).unwrap();
        assert_eq!(result,"739785")
    }

    #[test]
    fn day21_part2_test()  {
        let starting_positions = parse_input(true).unwrap();
        let result = part2(&starting_positions).unwrap();
        assert_eq!(result,"444356092776315")
    }
}