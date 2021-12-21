extern crate serde_scan;
use cached::proc_macro::cached;

// With three d3 we can get rolls in 27 different ways: (1,1,1), (1,1,2), (1,1,3) ...
// Of those, distinct sums are: 3, 4, 5, 6, 7, 8, 9
// and counts for those rolls are: 1, 3, 6, 7, 6, 3, 1
const DISTINCT_D3_ROLLS_AND_COUNTS: [(u32, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

const INPUT_FILE: &str = include_str!("../../inputs/day21.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    id: u32,
    position: u32,
    score: u32,
}

fn parse(input: &str) -> [Player; 2] {
    let mut lines = input.lines();
    let player_1_input = lines.next().unwrap();
    let player_2_input = lines.next().unwrap();

    let (player_id_1, player_1_pos) =
        serde_scan::scan!("Player {} starting position: {}" <- player_1_input).unwrap();
    let (player_id_2, player_2_pos) =
        serde_scan::scan!("Player {} starting position: {}" <- player_2_input).unwrap();

    [
        Player {
            id: player_id_1,
            position: player_1_pos,
            score: 0,
        },
        Player {
            id: player_id_2,
            position: player_2_pos,
            score: 0,
        },
    ]
}

fn part_1(input: &str) -> u32 {
    let mut players = parse(input);

    let mut deterministic_dice = 1;
    let mut dice_rolled = 0;
    let mut active_player: usize = 0;

    loop {
        let mut rolls = 0;
        for _ in 0..3 {
            rolls += deterministic_dice;
            deterministic_dice += 1;
            dice_rolled += 1;
        }

        // Treat position 10 as 0 so that we can simply use modulo 10
        players[active_player].position = (players[active_player].position + rolls) % 10;
        if players[active_player].position == 0 {
            players[active_player].score += 10;
        } else {
            players[active_player].score += players[active_player].position
        }

        if players[active_player].score >= 1000 {
            let losing_player = (active_player + 1) % 2;
            return players[losing_player].score * dice_rolled;
        }

        active_player = (active_player + 1) % 2;
    }
}

#[cached]
fn roll_dirac_dice(players: [Player; 2], active_player: usize) -> [usize; 2] {
    let mut wins = [0, 0];

    for (next_rolls, count) in DISTINCT_D3_ROLLS_AND_COUNTS {
        let universe_wins = play_quantum_turn(next_rolls, players, active_player);
        wins[0] += count * universe_wins[0];
        wins[1] += count * universe_wins[1];
    }

    wins
}

fn play_quantum_turn(rolls: u32, mut players: [Player; 2], active_player: usize) -> [usize; 2] {
    // Treat position 10 as 0 so that we can simply use modulo 10
    players[active_player].position = (players[active_player].position + rolls) % 10;
    if players[active_player].position == 0 {
        players[active_player].score += 10;
    } else {
        players[active_player].score += players[active_player].position
    }

    if players[active_player].score >= 21 {
        let mut wins = [0, 0];
        wins[active_player] += 1;
        return wins;
    }

    let next_player = (active_player + 1) % 2;

    roll_dirac_dice(players, next_player)
}

fn part_2(input: &str) -> usize {
    let players = parse(input);
    let active_player = 0;

    *roll_dirac_dice(players, active_player)
        .iter()
        .max()
        .unwrap()
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);

    let part_2_result = part_2(INPUT_FILE);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "Player 1 starting position: 4\n\
                 Player 2 starting position: 8"
            ),
            739785
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "Player 1 starting position: 4\n\
                 Player 2 starting position: 8"
            ),
            444356092776315
        );
    }
}
