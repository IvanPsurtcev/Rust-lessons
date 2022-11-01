use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub trait Logic {
    fn guess_number(&self) {
        println!("Загадай число!");
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("Загаданное число: {}", secret_number);

        let mut i_numbers = 0;
        loop {
            println!("Введи число игроков");
            let mut numbers = String::new();
        
            io::stdin()
                .read_line(&mut numbers)
                .expect("Failed to read line");
            let numbers: i32 = match numbers.trim().parse() {
                Ok(num) => num,
                Err(_) => -1,
            };
    
            if numbers == -1 {
                println!("Неправильное число игроков");
            } else {
                i_numbers = numbers;
                break;
            }
        }

        let mut i = 1;
        loop {
            if i <= i_numbers {
                println!("Догадались какое число загадано игрок {}?", i);
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                println!("{}-й игрок введите вашу догадку: {}", i, guess);
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Слишком мало!"),
                    Ordering::Greater => println!("Слишком много!"),
                    Ordering::Equal => {
                        println!("Вы выиграли игрок {}!", i);
                        break;
                    }
                }
                i += 1;
            } else {
                i = 1;
            }
        }
    }
}

pub struct Game;
    // number_game: i32,
    // number_players: i32,

impl Logic for Game {}







