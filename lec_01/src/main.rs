fn main() {
    // Variables are imutable by default
    // if mut not present, cannot re-assign it to 20
    let mut x = 10; 
    let name = "General Kenobi";

    // println! is a macro, not a function
    println!("Hello, there {}!", name);
    println!("x: {}", x);
    x = 20;
    println!("x: {}", x);
}
