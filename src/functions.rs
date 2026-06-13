pub fn run() {
    let result = add(5, 3);
    println!("Sum = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
