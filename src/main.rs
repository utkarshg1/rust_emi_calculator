use std::io;

fn main() {
    let p = get_input("Enter Principal in INR");
    let n = get_input("Enter Period in Years");
    let r = get_input("Enter Rate of Interest in % p.a.");

    let (emi, interest, total_amount) = emi_calculator(p, n, r);

    println!("EMI Calculated: {:.2} INR", emi);
    println!("Interest: {:.2} INR", interest);
    println!("Total Amount: {:.2} INR", total_amount);

    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn emi_calculator(principal: f64, years: f64, annual_rate: f64) -> (f64, f64, f64) {
    let months = years * 12.0;
    let monthly_rate = annual_rate / 1200.0;
    let factor = (1.0 + monthly_rate).powf(months);
    let emi = principal * monthly_rate * factor / (factor - 1.0);
    let total_amount = emi * months;
    let interest = total_amount - principal;

    (emi, interest, total_amount)
}

fn get_input(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{prompt}: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => break value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
