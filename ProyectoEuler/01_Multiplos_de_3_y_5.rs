fn main() {
    let mut sum = 0;
    for number in 1..1000 {
        if number%3 == 0 {
            println!("{}",number);
            sum = sum + number;
        } else if number%5 == 0 {
            println!("{}",number);
            sum = sum + number;
        }
    }

    // Print text to the console
    println!("{}",sum);
}