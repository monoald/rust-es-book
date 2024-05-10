fn main() {
    mutable_reference();
}

// "send" value as reference
fn reference() {
    let s1 = String::from("hola");

    let len = calcular_longitud(&s1);

    println!("La longitud de '{}' es {}.", s1, len);
}
fn calcular_longitud(s: &String) -> usize {
    s.len()
}

// Mutate reference
fn mutable_reference() {
    let mut s = String::from("hola");

    modificar(&mut s);
}
fn modificar(un_string: &mut String) {
    un_string.push_str(", mundo");
}

// We cannot create multiple mut ref or a static ref with a mut ref, unless the execution
// scope of the first ref is done.
fn multiple_invalid_ref() {
    let mut s = String::from("hola");

    let r1 = &s;
    let r2 = &s;
//    let r3 = &mut s; // Error

    println!("{r1}, {r2}, y {r2}");
}

fn multiple_valid_ref() {
    let mut s = String::from("hola");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} y {r2}"); // execution scope of r1 and r2 ended

    let r3 = &mut s;
    println!("{r3}");
}

// Dangling reference, the reference is pointing to a value that is no longer owns
//fn danglin ref() -> &String {
//    let s = String::fro("hola");

//    &s
//}
