use std::fs;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(EnumString, Clone, Copy, PartialEq)]
enum OpponentShapes {
    A = 1,
    B = 2,
    C = 3
}

#[derive(EnumString, Clone, Copy, PartialEq)]
enum MyShapes {
    X = 1,
    Y = 2,
    Z = 3
}

fn winner(os: OpponentShapes) -> MyShapes {
    return match os {
        OpponentShapes::A => MyShapes::Y,
        OpponentShapes::B => MyShapes::Z,
        OpponentShapes::C => MyShapes::X
    }
}
fn loser(os: OpponentShapes) -> MyShapes {
    return match os {
        OpponentShapes::A => MyShapes::Z,
        OpponentShapes::B => MyShapes::X,
        OpponentShapes::C => MyShapes::Y
    }
}

pub fn day02() {
    println!("starting day 02");

    let contents = fs::read_to_string("data/02_rockpaperscissors_strategy_guide.txt")
        .expect("Could not read file");

    let mut accu_part1 = 0;
    let mut accu_part2 = 0;

    for line in contents.split('\n') {
        let mut tokens = line.split_whitespace();
        let opponent_token = tokens.next().unwrap();
        let my_token = tokens.next().unwrap();

        let opponent = OpponentShapes::from_str(opponent_token).unwrap();
        let my = MyShapes::from_str(my_token).unwrap();

        let x = (opponent, my);
        let round_score_part1 = match x {
            _ if x == (OpponentShapes::A, MyShapes::Z) || x == (OpponentShapes::B, MyShapes::X) || x == (OpponentShapes::C, MyShapes::Y) => 0 + my as u32,
            _ if opponent as u32 == my as u32 => 3 + my as u32,
            _ => 6 + my as u32
        };
        accu_part1 += round_score_part1;

        let round_score_part2 = match my {
            MyShapes::X => loser(opponent) as u32 + 0,
            MyShapes::Y => opponent as u32 + 3,
            MyShapes::Z => winner(opponent) as u32 + 6
        };
        accu_part2 += round_score_part2;

    }

    println!("score part1 {:?}, score part2 {:?}", accu_part1, accu_part2);
}