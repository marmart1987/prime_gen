use std::io;
fn main() {
    println!("Enter a number:");
    loop {
        let mut highest_number_to_check_inclusive = String::new();
        io::stdin()
            .read_line(&mut highest_number_to_check_inclusive)
            .expect("Failed to read line");

        let highest_number_to_check_inclusive: u64 =
            match highest_number_to_check_inclusive.trim().parse() {
                Ok(val) => val,
                Err(_) => continue,
            };

        let mut list_of_primes = Vec::new();
        list_of_primes.push(2);
        for current_number in 3..=highest_number_to_check_inclusive {
            let mut failed: bool = false;
            for current_prime in &list_of_primes {
                if current_number % current_prime == 0 {
                    failed = true;
                    break;
                }
            }
            if !failed {
                list_of_primes.push(current_number);
            }
        }
        println!(
            "Here's all the primes up to {}: {:#?}",
            highest_number_to_check_inclusive, list_of_primes
        );
        break;
    }
}
