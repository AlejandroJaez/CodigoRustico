fn main() {
    let mut actual = 0;
    let mut ultimo = 1;
    let limite = 4000000;
    let mut suma = 1;
    let mut total: u128 = 0;
    while suma < limite {
        println!("{}",suma);
        if suma%2 == 0 {
            total = total + suma;
        }
        
        suma = actual + ultimo;
        actual = ultimo;
        ultimo = suma;
        
    }

    println!("suma: {}",total);
    

    // Print text to the console
    //println!("{}",sum);
}