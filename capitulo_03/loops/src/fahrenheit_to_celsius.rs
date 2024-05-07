pub fn f_to_c(temperature: i32) -> f32 {
    let a: f32 = (temperature) as f32 - 32.0;
    let b: f32 = 9.0 / 5.0;

    let celsius = a / b;

    println!("{b}");

    return celsius;
}
