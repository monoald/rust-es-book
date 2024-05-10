fn main() {
    {
        let s = String::from("hola");

        // s value is valid
    } // Drop is used in s, value is no longer valid

    // value is "moved" from s1 to s2, s1 is invalidated
    let s1 = String::from("hola");
    let s2 = s1;

    // println!("{}, mundo!", s1); // error s1 no longer has a value

    // use "clone" to "copy" de value
    let s1 = String::from("hola");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // no error

    // fixed values at compile time just "Copy" the value
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // no error

    let s2 = String::from("hola");

    let s3 = toma_y_devuelve(s2); // s2 value moved into function the moved to s3
}

// Non fixed compile time values are "moved" into the function
fn toma_y_devuelve(un_string: String) -> String {
    un_string // Non fixed compile time return values are "moved" to return the value
}
