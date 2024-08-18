use std::io;
fn main() {
    let p: f64 = get_input("Enter Principal in INR : ");
    let n: f64 = get_input("Enter Period in Years : ");
    let r: f64 = get_input("Enter Rate of Intrest in %p.a. : ");
    let results: (f64, f64, f64) = emi_calculator(p, n, r);
    println!("EMI Calculated : {} INR", results.0);
    println!("Intrest : {} INR", results.1);
    println!("Amount : {} INR", results.2);
}

fn emi_calculator(p: f64, n: f64, r: f64) -> (f64, f64, f64) {
    let n1: f64 = n * 12.0;
    let r1: f64 = r / 1200.0;
    let x: f64 = (1.0 + r1).powf(n1);
    let emi: f64 = p * r1 * x / (x - 1.0);
    let amt: f64 = emi * n1;
    let intr: f64 = amt - p;
    return (emi.round(), intr.round(), amt.round());
}

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{prompt} : ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let output: f64 = input.trim().parse().expect("Please enter valid number");
    output
}
