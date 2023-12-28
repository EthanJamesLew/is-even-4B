use is_even_macro::is_even;

fn main() {
    let check_even = is_even!(100); // Generates the if-else chain up to 100
    println!("Is 42 even? {}", check_even(42));
}

