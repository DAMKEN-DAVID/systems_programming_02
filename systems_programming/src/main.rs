fn main() {
    let tuple = (String::from("Hello"), 5);

    match tuple {
        (ref s, x) => {
            println!("String: {}, Number: {}", s, x);
        }
    }

    println!("Tuple's string: {}", tuple.0);  // Still accessible
}