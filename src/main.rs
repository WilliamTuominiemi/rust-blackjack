use rand::seq::SliceRandom;
fn main() {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut suits: [char; 4] = ['C', 'D', 'H', 'S'];
    suits.shuffle(&mut rng);

    let mut numbers: Vec<i32> = (1..13).collect();
    numbers.shuffle(&mut rng);

    deal_card(suits, numbers.clone(), rng.clone());
    deal_card(suits, numbers.clone(), rng.clone());
}

fn deal_card(suits: [char; 4], numbers: Vec<i32>, mut rng: rand::prelude::ThreadRng) {
    let suit = suits.choose(&mut rng).unwrap();
    let number = numbers.choose(&mut rng).unwrap();

    println!("{} {}", suit, number);
}
