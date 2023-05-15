fn main() {
    say_hello();

    // The compiler will understand the usage of x1, x2 in say_the_number
    // and make them u8 type
    let x1 = 32;
    let x2 = 64;
    say_the_sum(x1, x2);
    say_number(x1 as i32);

    let result = squared(13);
    println!("result is: {result}");

    let result2 = squared2(13);
    println!("result is: {:?}", result2);

    let celsius: f64 = 23.0;
    println!("{celsius} C is {} in F", celsius_to_farenheit(celsius));
}

fn say_hello() {
    println!("Hello, world!");
    say_number(13);
}

fn say_number(number: i32) {
    println!("number is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    println!("sum of {a} and {b} is {}", a+b);
}

fn squared(x: i32) -> i32 {
    println!("squaring {x}");
    // if no ; at the end, automaticaly return expression
    x*x
}

fn squared2(x: i32) -> (i32, i32) {
    println!("squaring {x}");
    (x, x*x)
}

fn celsius_to_farenheit(c: f64) -> f64 {
    return (1.8* c) + 32.0
}