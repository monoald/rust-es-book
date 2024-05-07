fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let condition = true;
    // Should return the same type value
    let othernumber = if condition { 5 } else { 6 };

    println!("The value of the number is: {othernumber}");
}
