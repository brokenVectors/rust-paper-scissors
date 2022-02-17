use rand::Rng;
use std::io;

fn player_win() {
    println!("You win!");
}

fn player_loss() {
    println!("You lose!");
}

fn draw() {
    println!("Draw!");
}

fn main() {
    println!("Rock, paper, scissors, shoot!");
    let my_move_number: i32 = rand::thread_rng().gen_range(1..31);
    let mut my_move = String::new();
    if my_move_number < 30 {
        my_move = String::from("rock");
    }

    if my_move_number < 20 {
        my_move = String::from("paper");
    }

    if my_move_number < 10 {
        my_move = String::from("scissors");
    }

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            user_input = user_input.trim().to_string().to_lowercase();
        }

        Err(e) => {
            println!("{:?}", e);
        }
    }
    println!("I played {}!", my_move);
    // Rock beats scissors, scissors beats paper, paper beats rock
    // if if if if if if if
    if user_input.eq("rock") && my_move.eq("scissors") {
        player_win();
    }
    if user_input.eq("scissors") && my_move.eq("paper") {
        player_win();
    }
    if user_input.eq("paper") && my_move.eq("rock") {
        player_win();
    }
    if my_move.eq("rock") && user_input.eq("scissors") {
        player_loss();
    }
    if my_move.eq("scissors") && user_input.eq("paper") {
        player_loss();
    }
    if my_move.eq("paper") && user_input.eq("rock") {
        player_loss();
    }
    if my_move.eq(&user_input) {
        draw();
    }
}
