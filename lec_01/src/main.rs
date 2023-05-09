fn main() {
    // Variables are imutable by default
    // if mut not present, cannot re-assign it to 20
    let mut x: u8 = 254; 
    let y: f64 = 0.5456834013658346;
    let name = "General Kenobi";

    // println! is a macro, not a function
    println!("Hello, there {}!", name);
    
    println!("x: {}", x);
    x += 1;
    println!("x: {}", x);

    println!("y: {}", y);
}
