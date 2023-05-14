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

    let a= 10.0;
    let b = 3.0;
    let c = a / b;

    println!("c is {c:10.3}\na is {a}\nagain c is {c:.3}");

    let mut value = 0b1111_0101u8;
    println!("Value is {value}");
    println!("Value is {value:08b}");

    value = !value;
    println!("Value is {value:08b}");
    
    value = value & 0b1111_0111;
    println!("Value is {value:08b}");
    println!("Bit 6 is {:.08b}", value & 0b0100_0000);

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';

    println!("{letter}\t{number}\t{finger}");

    let x1 = 13;
    let x2 = 2.3;
    let x3: f32 = 120.0;

    let avg = (x1 as f64 + x2 as f64 + x3 as f64) / 3.0;
    assert_eq!(avg, 45.1);
    println!("Test passed!");

    // Array must be same data type and have fixed lenght
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    println!("first letter is: {}", letters[0]);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index = numbers.len();
    println!("number at index {} is: {}", index - 1, numbers[index - 1]);

    let A = [[1, 2, 3], [4, 5, 6]];
    println!("A element: {}", A[0][2]);

    // Tuple
    let stuff = (10, 3.14, 'x');
    println!("First stuff is {}", stuff.0);

    let (s1, s2, s3) = stuff;
    println!("s1: {s1}, s2: {s2}, s3: {s3}");
}
