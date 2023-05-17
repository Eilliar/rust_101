fn main() {
    if true {
        let planet: &str = "Earth";
        println!("planet is {planet}");
    }
    // Bellow will yield error, since planet is out of scope
    // println!("planet is {planet}");

    // Shadowing also work only within a scope
    let planet: &str = "Earth";
    {
        println!("planet is {planet}");
        let mut planet = 4;
        println!("planet is {planet}");
    }
    println!("planet is {planet}");

    // in Rust Program Memory is divided

    // Stack (add in a LIFO)
    // Push and pop data very quickly
    // Access data very quickly
    // Small size
    // All data mus have a known, fixed size

    // Heap
    // Uses Pointer
    // Adding and acessing data is slower than the stack
    // Dynamically add and remove data

    // String Literal (defined with "")
    // Hard-coded into the executable
    // Immutable
    // Must be known before compilation

    // String Type
    // Allocated on the Heap Memory
    // Mutable
    // Dynamically generated at runtime
    let mut message: String = String::from("Earth");
    message.push_str(" is home.");

    println!("{message}");

    // Ownership
    // Variables are responsible for freeing their own resources
    // Rules
    // 1. Every value is "owned" by one, and only one, variable at a time
    // 2. When the owning variable goes out os scope, the value is dropped
    // Advantage:
    // Safe
    // Efficient
    // Disadvantage
    // Requires understanding of ownership
    let outer_planet: String;
    {
        // Ownership apply to data in Heap Memory
        let inner_planet: String = String::from("Mercury");
        println!("inner_planet is {inner_planet} at {:p}", &inner_planet);
        // Once out of scope, inner_planet will cease to exist, so pass ownership to outer_planet
        // rust performs a 'move' and invalidate inner_planet ownership to the data in Heap
        outer_planet = inner_planet;
        // Line bellow will yield error
        // to avoid this, make line above outer_planet = inner_planet.clone();
        // Copying data in Stack occurs implicity, while in Heap cloning must be done explicitly
        // println!("innerplanet is {inner_planet} at {:p}", &inner_planet);
    }
    println!("outer_planet is {outer_planet} at {:p}", &outer_planet);

    let rocket_fuel_int: i32 = 1;
    process_fuel_int(rocket_fuel_int);
    println!("rocket fuel is {rocket_fuel_int} at {:p}", &rocket_fuel_int);

    let rocket_fuel_str: String = String::from("RP-1");
    println!("rocket fuel is {rocket_fuel_str} at {:p}", &rocket_fuel_str);
    let rocket_fuel_str: String = process_fuel_str(rocket_fuel_str);
    println!("rocket fuel is {rocket_fuel_str} at {:p}", &rocket_fuel_str);


}

fn process_fuel_int(mut propellant: i32){
    propellant += 1;
    println!("processing propellant {propellant}... at {:p}", &propellant);
}

fn process_fuel_str(propellant: String) -> String {
    println!("processing propellant {propellant}... at {:p}", &propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}