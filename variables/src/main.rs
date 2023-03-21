fn main() {
    let x = 5;
    let mut x = x + 1;

    {
        let x = x + 4;
        println!("The value of inner scope x is: {x}");
    }
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    numeric_operations();

    tuples();
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}\n");
}

fn tuples() {
    println!("Tuples:");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, _, _) = tup;

    println!("The value of x in {:?} is: {x}", tup);

    let five = tup.0;
    println!("The value of first index in {:?} is: {five}", tup);
}
