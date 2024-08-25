fn main() {
    let s1: String = String::from("hello");

    let len0: usize = calculate_length(&s1);
    let len1: usize = calculate_length(&s1);

    println!("The length of '{s1}' is {len0}, {len1}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
