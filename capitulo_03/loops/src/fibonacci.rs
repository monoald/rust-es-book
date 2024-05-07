pub fn fibonacci(mut n: i32) {
    let mut n1 = 0;
    let mut n2 = 1;

    while n >= 0 {
        println!("{n1}");
        let temp = n1 + n2;
        n1 = n2;
        n2 = temp;
        n -= 1;
    }
}
