fn main() {
    // inmutable and mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let y = 6;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    print!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
}
