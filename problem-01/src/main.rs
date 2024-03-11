fn main() {
    let sum_of_multiples = (1..1000).filter(|v| v % 5 == 0 || v % 3 == 0).sum::<i32>();
    println!("Sum of multiples: {sum_of_multiples}")
}
