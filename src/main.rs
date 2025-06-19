use rand::seq::SliceRandom;
use std::io;

fn main() {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut suits: [char; 4] = ['C', 'D', 'H', 'S'];
    suits.shuffle(&mut rng);

    let mut numbers: Vec<i32> = (1..13).collect();
    numbers.shuffle(&mut rng);

    let mut total = 0;
    let mut dealer_total = 0;

    print!("To you: ");
    total += deal_card(total);
    print!("To dealer: ");
    dealer_total += deal_card(total);
    print!("To you: ");
    total += deal_card(total);

    println!("You're at {}", total);
    println!("Dealers at {}", dealer_total);

    let mut stand = false;

    while !stand && total < 21 {
        println!("H to hit, S to stand");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "S" || input == "s" {
                    stand = true;
                } else if input == "H" || input == "h" {
                    print!("To you: ");
                    total += deal_card(total);
                    println!("You're at {}", total);
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }

    if total > 21 {
        println!("You bust");
    } else if total == 21 {
        println!("Blackjack, you won");
    } else {
        println!("You stand at {}", total);
        println!("Dealers turn");

        while dealer_total <= 17 {
            print!("To dealer: ");
            dealer_total += deal_card(total);
        }
        println!("Dealers at {}", dealer_total);

        if dealer_total > 21 {
            println!("Dealer busts, you win!");
        } else if total > dealer_total {
            println!("You win");
        } else if total == dealer_total {
            println!("Draw");
        } else {
            println!("You lose");
        }
    }
}

fn deal_card(total: i32) -> i32 {
    let card_number = get_card();

    if card_number == 1 && (total - 11) < 21 {
        return 11;
    } else if card_number == 11 || card_number == 12 || card_number == 13 {
        return 10;
    } else {
        return card_number;
    }
}

fn get_card() -> i32 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut suits: [char; 4] = ['C', 'D', 'H', 'S'];
    suits.shuffle(&mut rng);

    let mut numbers: Vec<i32> = (1..13).collect();
    numbers.shuffle(&mut rng);

    let suit = suits.choose(&mut rng).unwrap();
    let number = numbers.choose(&mut rng).unwrap();

    println!("{}{}", suit, number);

    return *number;
}
