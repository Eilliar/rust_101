// References

// Concepts:
// 1. Borrowing references using &
// 2. Dangling reference (return &<variable_name>)
// 3. Slices

fn main(){
    let rocket_fuel_1 = String::from("RP-1");
    let (rocket_fuel_1, lenght) = process_fuel_1(rocket_fuel_1);
    println!("rocket_fuel_1 is {rocket_fuel_1} with lenght {lenght} @ {:p}", &rocket_fuel_1);

    let rocket_fuel_2 = String::from("RP-2");
    let lenght = process_fuel_2(&rocket_fuel_2);
    println!("rocket_fuel_2 is {rocket_fuel_2} with lenght {lenght} @ {:p}", &rocket_fuel_2);

    let mut rocket_fuel_3 = String::from("RP-3");
    let lenght = process_fuel_3(&mut rocket_fuel_3);
    println!("rocket_fuel_3 is {rocket_fuel_3} with lenght {lenght} @ {:p}", &rocket_fuel_3);

    let rocket_fuel_4 = produce_fuel();
    println!("rocket_fuel_4 is {rocket_fuel_4} @ {:p}", &rocket_fuel_4);

    // Slices
    let message = String::from("Greetings from Earth!");
    println!("{message} @ {:p}", &message);
    let last_word = &message[15..];
    println!("last word is {last_word} @ {:p}", &last_word);
    let first_word = get_first_word(&message);
    println!("first word is {first_word} @ {:p}", &first_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("planets are : {:?} @ {:p}", planets, &planets);
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are: {:?} @ {:p}", inner_planets, inner_planets);

    println!("------------------ Challenge ------------------");
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    let test3 = String::from("There is space to the rear.   ");
    assert_eq!(trim_spaces(&test3), "There is space to the rear.");
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    let test5 = "       ";
    assert_eq!(trim_spaces(test5), "");

    println!("All tests passed!");

}

fn process_fuel_1(propellant: String) -> (String, usize) {
    println!("processing propellant {propellant} @ {:p}", &propellant);
    let length = propellant.len();
    (propellant, length)
}

// Borrow the reference only (like getting the pointer in C++)
fn process_fuel_2(propellant: &String) -> usize {
    println!("processing propellant {propellant} @ {:p}", propellant);
    let lenght = propellant.len();
    lenght
}

fn process_fuel_3(propellant: &mut String) -> usize {
    println!("processing propellant {propellant} @ {:p}", propellant);
    propellant.push_str(" [is highly flamable]");
    let lenght = propellant.len();
    lenght
}

// Dangling reference example
// The compiler will not let the bellow happen, since new_fuel goes out of scope
// fn produce_fuel() -> &String {
//     let new_fuel = String::from("RP-Alpha");
//     &new_fuel
// }
fn produce_fuel() -> String {
    let new_fuel = String::from("RP-Alpha");
    new_fuel
}

// &String => String reference is different from
// &str => string slice
fn get_first_word(message: &String) -> &str {
    let bytes = message.as_bytes();

    for (index, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &message[..index];
        }
    }
    // No space was found, it's  a single word message
    &message
}

fn trim_spaces(message: &str) -> &str {
    let bytes = message.as_bytes();

    let (mut start_index, mut end_index) = (0, 0);

    for (index, &item) in bytes.iter().enumerate(){
        if item != b' '{
            start_index = index;
            break;
        }

    }
    for (index, &item) in bytes.iter().rev().enumerate(){
        if item != b' ' {
            end_index = bytes.len() - index;
            break;
        }
    }
    
    &message[start_index..end_index]
}