use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let dest = &args[1];
    let mut sum: u128 = 0;
    for number in 1..dest.parse::<u128>().unwrap() {
        if number%3 == 0 {
            //println!("{}",number);
            sum = sum + number;
        } else if number%5 == 0 {
            //println!("{}",number);
            sum = sum + number;
        }
    }

    // Print text to the console
    println!("{}",sum);
}