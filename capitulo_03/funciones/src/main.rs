fn main() {
    println!("Hello, world!");

//    function_parameters(5);
//    print_labeled_measurement(5, 's');
//    sentencias_expresiones();

//    let x = with_return();
//    println!("The value of x is: {x}");

    let x = plus_one(1);
    println!("The value of x is: {x}");

}

fn another_function() {
    println!("Another function.");
}

fn function_parameters(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn sentencias_expresiones() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn with_return() -> i32 {
    6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
