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

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array {:?}", numbers);

    let fruits: [&str; 3] = ["apples", "banana", "orange"];
    println!("Fruit: {} {} {}", fruits[0], fruits[1], fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mixed Tuple {:?}", my_mix_tuple);

    //Slices -- dynamically sized
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slices {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Harry Potter".to_string(), &"Elephant Book".to_string()];
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

    let _X = {
        let price = 5;
        let qty = 10;
        price * qty
    };

    println!("Result is: {}", _X);

    println!("Add these numbers: {}", add(3, 4));

    let s1 = String::from("RUST");
    println!("The length of {} is {}", s1, calc_length(&s1));

    let mut _num: i32 = 5;
    let mut _copy: &i32 = &_num;

    println!("{}, {}", _num, _copy); //output: 5, 5 (_copy is a copy of _num)
    let _num: i32 = 7;
    println!("{}, {}", _num, _copy); //output: 7, 5 (_copy retains the original _num)
    let mut _copy: i32 = _num;

    println!("{}, {}", _num, _copy); //output: 7, 5 (_copy retains the original _num)

    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    account.check_balance();
    account.withdraw(30.33);
    account.check_balance();

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Value of x in scope {x}");
    }
    println!("Value of x out of scope {x}");

    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You aren't old enough to drive!");
    }

    //loop runs forever until told to stop
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        println!("Current Count: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Final result: {result}");

    let a = [1, 2, 3, 4, 5, 6];
    for element in a {
        println!("{element}");
    }

    let mut user1: User = User {
        active: true,
        username: String::from("username"),
        email: String::from("user@test.com"),
        sign_in_count: 30,
    };

    user1.email = String::from("Newuser@test.com");

    println!("{} email is: {}", user1.username, user1.email);

    let mut user2: User = build_user(String::from("reishi@reishi.com"), String::from("reishi"));

    println!("{} email is: {}", user2.username, user2.email);
    let car = Sedan;
    let suv = SUV;

    road_trip(&car);
    road_trip(&suv);
}

fn hello_world() {
    println!("Hello Rust!");
}

fn tell_height(height: i32) {
    println!("Height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old. My height is {}",
        name, age, height
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calc_length(s: &String) -> usize {
    s.len()
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account, owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has balance of {}",
            self.owner, self.balance
        );
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

struct Sedan;
impl LandCapable for Sedan {}

struct SUV;
impl LandCapable for SUV {}

trait LandCapable {
    fn drive(&self) {
        println!("Vehicle is driving");
    }
}


fn road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}