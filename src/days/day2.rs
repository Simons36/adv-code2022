use std::vec;

use crate::util;

const ROCK_OPP : &str = "A";
const PAPER_OPP : &str = "B";
const SCISSORS_OPP : &str = "C";

const ROCK_USER : &str = "X";
const PAPER_USER : &str = "Y";
const SCISSORS_USER : &str = "Z";

const WIN_USER : &str = "Z";
const DRAW_USER : &str = "Y";
const LOSS_USER : &str = "X";

pub fn function () -> Result<[String; 2], String>{
    //part 1
    let vect = util::read_lines("input/input2.txt");

    let vect2 = vect.clone();

    let mut score1 = 0;

    for str in vect{
        let mut play = str.split_whitespace();

        let opponent_play = play.next().unwrap();
        let user_play = play.next().unwrap();

        score1 += check_winner(opponent_play, user_play);
        
    }

    //part 2
    let mut score2 = 0;

    for str in vect2{
        let mut play = str.split_whitespace();

        let opponent_play = play.next().unwrap();
        let expected_result = play.next().unwrap();

        score2 += check_winner(opponent_play, get_expected_play(opponent_play, expected_result).as_str());
    }

    return Ok([score1.to_string(), score2.to_string()]);
    
}

fn check_winner(opponent_play : &str, user_play : &str) -> i32{
    let mut add_points = 0;

    if user_play.eq(ROCK_USER){
        add_points = 1;
    }else if user_play.eq(PAPER_USER){
        add_points = 2;
    }else{
        add_points = 3;
    }

    //draw cases
    if opponent_play.eq(ROCK_OPP) && user_play.eq(ROCK_USER) { return 3 + add_points; }

    if opponent_play.eq(PAPER_OPP) && user_play.eq(PAPER_USER) { return 3 + add_points; }

    if opponent_play.eq(SCISSORS_OPP) && user_play.eq(SCISSORS_USER) {return 3 + add_points; }

    //defeat cases
    if opponent_play.eq(ROCK_OPP) && user_play.eq(SCISSORS_USER) { return add_points; }

    if opponent_play.eq(PAPER_OPP) && user_play.eq(ROCK_USER) { return add_points; }

    if opponent_play.eq(SCISSORS_OPP) && user_play.eq(PAPER_USER) { return add_points; }

    //otherwise win
    return 6 + add_points;
}

fn get_expected_play(opponent_play : &str, expected_result : &str) -> String{
    if opponent_play.eq(ROCK_OPP){
        if expected_result.eq(WIN_USER){
            return PAPER_USER.to_string();
        }

        if expected_result.eq(DRAW_USER){
            return  ROCK_USER.to_string();
        }

        return SCISSORS_USER.to_string();
    }

    if opponent_play.eq(PAPER_OPP){
        if expected_result.eq(WIN_USER){
            return SCISSORS_USER.to_string();
        }

        if expected_result.eq(DRAW_USER){
            return PAPER_USER.to_string();
        }


        return ROCK_USER.to_string();
    }

    //opponent is scissors

    if expected_result.eq(WIN_USER){
        return ROCK_USER.to_string();
    }

    if expected_result.eq(DRAW_USER){
        return SCISSORS_USER.to_string();
    }

    return PAPER_USER.to_string();
}