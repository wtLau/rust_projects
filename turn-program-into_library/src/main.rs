struct Fern {
    size: f64,
    growth_rate: f64,
}

impl Fern {
    // Simulate a fern growing for one day
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

// Run a fern simulation for some number of days
fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

fn main() {
    println!("Hello, world!");
    let mut fern = Fern {
        size: 1.00,
        growth_rate: 0.1,
    };

    run_simulation(&mut fern, 100);
    println!("fianl fern size: {}", fern.size);
}
