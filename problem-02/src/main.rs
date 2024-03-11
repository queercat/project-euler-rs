fn generate_fibonacci(max_value: i32) -> Vec<i32> {
    let mut values = [1, 1].to_vec();
    let mut index = 1;

    while values[index] < max_value {
        values.push(values[index - 1] + values[index]);
        index += 1;
    }

    values
}


fn main() {
    let sum_of_even_valued_terms = generate_fibonacci(4_000_000).iter().filter(|v| *v % 2 == 0).sum::<i32>();
    println!("Sum of even valued terms: {sum_of_even_valued_terms}")
}
