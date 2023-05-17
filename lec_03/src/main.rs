fn main() {
    let mut x = 5;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    }
    else if x < y {
        println!("x is NOT greater than y");
    }
    else {
        println!("x is equal to y");
    }

    let make_x_odd = true;
    x = if make_x_odd {1} else {2};

    println!("x is {x}");

    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10; // break can return a value
        }
        count += 1;
        println!("count is {count}");
    };

    println!("result is {result}");

    let letters = ['h', 'e', 'l', 'l', 'o', '\n'];
    count = 0;
    while count < letters.len() {
        println!("letter at {count} is {}", letters[count]);
        count += 1;
    }

    for (i, &item) in letters.iter().enumerate() {
        if item == '\n' {
            break;
        }
        println!("letter at {i} is {item}");
    }

    let mut A = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in A.iter_mut() {
        for number in row.iter_mut(){
            *number += 10;
            print!("{number}\t");
        }
        println!();
    }

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for &number in numbers.iter(){
        mean += number as f64;
        if number > max {
            max = number;
        }
        if number < min {
            min = number;
        }
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");

}
