use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    loop {
        println!("rock scissors paper");
        let rand_rsp = rand::thread_rng().gen_range(1..3);
        let mut player_rsp = String::new();

        io::stdin().read_line(&mut player_rsp).expect("입력실패");

        if rand_rsp == 1 {
            println!("Rock");
        } else if rand_rsp == 2 {
            println!("scissors");
        } else {
            println!("paper");
        }

        let player_rsp: u32 = player_rsp.trim().parse().expect("Please type number");

        match player_rsp.cmp(&rand_rsp) {
            Ordering::Equal => println!("Draw"),
            Ordering::Greater => println!("Lose"),
            Ordering::Less => {
                println!("win");
                break;
            }
        }
    }
}
