use rand::seq::SliceRandom;
fn main() {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut suits: [char; 4] = ['C', 'D', 'H', 'S'];
    suits.shuffle(&mut rng);

    let mut numbers: Vec<i32> = (1..13).collect();
    numbers.shuffle(&mut rng);

    let mut total = 0;
    let mut dealer_total = 0;

    // Starting position
    print!("To you: ");
    total += deal_card(total);
    print!("To dealer: ");
    dealer_total += deal_card(total);
    print!("To you: ");
    total += deal_card(total);

    println!("You're at {}", total);
    println!("Dealers at {}", dealer_total);

    if total == 21 {
        println!("Blackjack, you won");
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
