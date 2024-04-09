use turn_program_into_library::{run_simulation, Fern};

fn main() {
    println!("Hello, world!");
    let mut fern = Fern {
        size: 1.00,
        growth_rate: 0.1,
    };

    run_simulation(&mut fern, 100);
    println!("fianl fern size: {}", fern.size);
}
