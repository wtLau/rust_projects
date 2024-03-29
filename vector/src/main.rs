fn main() {
    let languages: Vec<String> = std::env::args().skip(1).collect();

    // Can use for loop to iterarte over a vector
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
