fn main() {
    let mut x: i32 = -42;
    let y: u64 = 42;

    println!("signed: {} unsigned: {}", x, y);

    x = -15;
    println!("signed: {} unsigned: {}", x, y);

    let is_snowing: bool = true;
    println!("Is it snowing?: {}", is_snowing);

    let letter: char = 'a';
    println!("First letter: {}", letter);

    let numbers : [i32; 5]= [1,2,3,4,5];
    println!("Array {:?}", numbers);

    let fruits: [&str; 3] = ["apples", "banana", "orange"];
    println!("Fruit: {} {} {}", fruits[0], fruits[1], fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("Mixed Tuple {:?}", my_mix_tuple);

    //Slices -- dynamically sized
    let number_slices : &[i32] = &[1,2,3,4,5];
    println!("Number Slices {:?}", number_slices);

    let animal_slices : &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices {:?}", animal_slices);

    let book_slices : &[&String] = &[&"Harry Potter".to_string(), &"Elephant Book".to_string()];
    println!("Book Slices {:?}", book_slices);

    // Strings vs String Slices (&str)
    // Strings are growable, mutable, owned string type
    let mut rust_statement: String = String::from("Hello, ");
    rust_statement.push_str("World!");
    println!("Rust Lang says: {}", rust_statement);

    let string: String = String::from("Hi there");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

    hello_world();
    tell_height(182);
    human_id("George", 30, 190.2);

}

fn hello_world() {
    println!("Hello Rust!");
}

fn tell_height(height: i32) {
    println!("Height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old. My height is {}", name, age, height);
}