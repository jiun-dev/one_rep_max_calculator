use std::io;

fn main() {
    println!("Enter your 10RM: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let ten_rm: f32 = input.trim().parse().unwrap();

    let one_rm = calculate_one_rep_max(ten_rm);
    println!("Your one rep max is {}kg", one_rm);
}

fn calculate_one_rep_max(ten_rm : f32) -> f32 {
    (ten_rm / (1.0278 - (0.0278 * 10.0))) * 1.0
}
