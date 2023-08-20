use rand::random;
use rps::RPS;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

fn main() {
    let mut rounds: u8 = 0;
    let mut score: i8 = 0;

    loop {
        if rounds >= 3 {
            break;
        }

        let computer: RPS = random();
        let user = read_user();

        match user.cmp(&computer) {
            Ordering::Equal => {
                println!("Ничья: {} = {}", user, computer);
                continue;
            }
            Ordering::Less => {
                println!("Ты проиграл: {} < {}", user, computer);
                score -= 1;
            }
            Ordering::Greater => {
                println!("Ты выйграл: {} > {}", user, computer);
                score += 1;
            }
        }

        rounds += 1;
    }

    match score.cmp(&0) {
        Ordering::Equal => println!("Игра окончилась ничьей."),
        Ordering::Less => println!("Победил компьютер."),
        Ordering::Greater => println!("Игрок победил."),
    }
}

fn read_user() -> RPS {
    loop {
        print!("Вводи (к/н/б): ");
        stdout().flush().expect("Не получилось сбросить STDOUT.");
        let mut buf = String::new();

        if stdin().read_line(&mut buf).is_ok() {
            match RPS::from_str(&buf) {
                Ok(rps) => return rps,
                Err(error) => println!("{}: {}", error, buf),
            }
        }
    }
}