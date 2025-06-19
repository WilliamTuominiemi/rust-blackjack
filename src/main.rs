use rand::seq::SliceRandom;
fn main() {
    let mut rng = rand::thread_rng();

    let mut suits = ['C', 'D', 'H', 'S'];
    suits.shuffle(&mut rng);

    let mut numbers: Vec<i32> = (1..13).collect();
    numbers.shuffle(&mut rng);

    let suit = suits.choose(&mut rng).unwrap();
    let number = numbers.choose(&mut rng).unwrap();

    println!("{} {}", suit, number);
}
