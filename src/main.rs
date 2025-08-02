fn main() {
    vars();
    data_types();
    compound_data_types();
    add(1, 2);
    control_flow();
    loops();
    owner();
    moved();
    ownership_functions();
    takes_ownership(String::from("Hello"));
    makes_copy(5);
    borrow_example();
    slice_example();
    string_example();
    concatenation_example();
    index_strings();
    struct_example();
    build_user(
        String::from("johndoe@example.com"),
        String::from("John Doe"),
    );
    tuple_struct();
    calc_area_rect(&Rect {
        width: 30,
        height: 50,
    });
    calc_area_rect2(&Rect {
        width: 30,
        height: 50,
    });
    enum_example();
    option_example();
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Penny);
    if_let_example();
    // result_example();
    operation_example();
}

fn vars() {
    // Variables are immutable by default in Rust
    // Declare a mutable(mut) variable to allow changes
    let mut x: i32 = 5;
    println!("x has the value {}", x);
    x = 6;
    println!("x has the value {}", x);

    // Shadowing allows you to reuse a variable name
    let y: i32 = 10;
    println!("y has the value {}", y);
    let y: &str = "ten";
    println!("y has the value {}", y);

    // Constants are immutable and must have a type annotation
    const EXAMPLE: i32 = 100_000_000; // Using underscores for readability
    println!("EXAMPLE has the value {}", EXAMPLE);
}

fn data_types() {
    // Integers
    // Rust defaults to i32
    // Signed integers can be positive or negative, Unsigned integers can only be positive
    let a: i32 = 42; // i8, i16, i32, i64, i128, isize(architecture dependent 32/64 bit)
    let a1: i32 = 98_222; // Decimal
    let a2: i32 = 0xff; // Hexadecimal
    let a3: i32 = 0o77; // Octal
    let a4: i32 = 0b1111_0000; // Binary
    let a5: u8 = b'A'; // Byte (ASCII value of 'A')
    let b: f64 = 3.14; // Floating point

    let c: bool = true; // Boolean
    let d: char = 'z'; // Character
    let e: &str = "Hello"; // String slice

    println!(
        "a: {}, a1: {}, a2: {}, a3: {}, a4: {}, a5: {}, b: {}, c: {}, d: {}, e: {}",
        a, a1, a2, a3, a4, a5, b, c, d, e
    );
}

fn compound_data_types() {
    // Tuples
    let tuple: (i32, f64, &str) = (42, 3.14, "Hello");
    println!("Tuple: {:?}", tuple);
    // We can get values from a tuple by two ways
    // 1. Destructuring
    let (x, y, z) = tuple;
    println!("Destructured: x: {}, y: {}, z: {}", x, y, z);

    // 2. Indexing
    println!("Indexed: x: {}, y: {}, z: {}", tuple.0, tuple.1, tuple.2);

    // Arrays
    // Arrays are fixed-size and must have the same type for all elements
    // For Dynamic arrays, use Vec<T>
    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
}

// Functions
// In Rust, functions are declared using the `fn` keyword
// Are in snake_case
fn add(x: i32, y: i32) -> i32 {
    println!("Adding {} and {}", x, y); // This is a statement, because this dont return anything
    // return x + y; // We can return the value of a function using the `return` keyword
    x + y // Or just return the value
}

// Control flow
fn control_flow() {
    let number: i32 = 5;

    if number < 10 {
        println!("Number is less than 10");
    } else if number < 20 {
        println!("Number is between 10 and 20");
    } else {
        println!("Number is 20 or more");
    }

    let condition: bool = true;
    let numb: i32 = if condition { 5 } else { 10 }; // if expressions can return values
    println!("numb is {}", numb);

    // Match statement
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Something else"), // _ is a catch-all pattern
    }
}

// Loops
// Rust has three types of loops: loop, while, and for
fn loops() {
    // loop {
    //     println!("Looping forever");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter > 5 {
    //         break counter;
    //     }
    //     println!("Counter: {}", counter);
    // };

    // println!("Result: {}", result);

    // while counter != 5 {
    //     counter += 1;
    //     println!("Counter: {}", counter);
    // }

    for i in 1..=5 {
        println!("Counter: {}", i);
    }

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Element: {}", element);
    }
}

// Memory Safety

// Ownership -> ownership is a way to manage memory in Rust
// It allows us to have memory safety without a garbage collector
// During runtime, rust makes certain decisions based on if our memory is stored on the stack or the heap
// During runtime, we have access to both the stack and the heap
// Stack: Fixed size(calculated at compile time), fast access, LIFO (Last In First Out)
// Heap: Dynamic size, slower access, FIFO (First In First Out)
fn owner() {
    // A gets executed first, then we push A onto the stack
    fn a() {
        // A inicializes x and y
        // X is a reference to a string literal, wich is stored in our binary
        let x: &str = "Hello";
        println!("A: {}", x);
        // Y is a signed 32 bit integer which is a fixed size, so can be stored directly on the stack frame
        let y: u32 = 22;
        println!("A: {}", y);
        // A executes B, then we push B onto the stack
        b();
    }
    a();

    // Another stack frame is created for B
    fn b() {
        // B has its on variable x
        // X is a String, which is a dynamic size, so can be stored on the heap
        // The heap passes back a pointer, this is what we actually store on the stack
        let x: String = String::from("World");
        // When B finishes executing, it pops off the stack
        println!("B: {}", x);
    }
    b();

    // ----- OWNERSHIP RULES -----
    // 1. Each value has a variable that`s called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped

    {
        // s is not valid here, its not yet declared
        // s ⬇️ is a reference to a string literal, wich is stored in our binary
        let s0: &str = "Hello"; // s is valid from this point forward
        println!("s0: {}", s0);

        // s ⬇️ is a String, which is a dynamic size, so can be stored on the heap
        let s: String = String::from("World"); // s is valid from this point forward
        // do stuff with s
        println!("s: {}", s);
    } // this scope is now over, s is no longer valid
}

fn moved() {
    let x: i32 = 5;
    let y: i32 = x; // Copying the value of x into y
    println!("y: {}", y);

    // Rust default to moving the value, cloning its more expensive
    let s1: String = String::from("Hello");
    let s2: String = s1; // Move (not shallow copy)
    let s3: String = s2.clone(); // Cloning the value of s2 into s3
    println!("s2: {}, s3: {}", s2, s3);
}

// Ownership & Functions
fn ownership_functions() {
    let s: String = String::from("Hello");
    takes_ownership(s);
    // println!("s: {}", s); // This would cause an error, because s is no longer valid after being moved

    let x: i32 = 5;
    makes_copy(x);
    println!("x: {}", x); // This is valid, because i32 is a Copy type

    let s2: String = gives_ownership(); // s2 takes ownership of the returned value
    println!("s2: {}", s2); // s2 is valid here, because it owns the value

    let s3: String = takes_and_gives_back_ownership(s2); // s3 takes ownership of the returned value
    println!("s3: {}", s3); // s3 is valid here, because it owns the value
}

fn takes_ownership(some_string: String) {
    println!("Taking ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Making a copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Hello");
    some_string
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
    println!("Taking and giving back ownership: {}", some_string);
    some_string // 
}

// References
// References are a way to borrow values without taking ownership
// Rules of References
// 1. At any given time, you can have either (but not both) a mutable reference or any number of immutable references.
// 2. References must always be valid.
fn borrow_example() {
    let mut s: String = String::from("Hello");
    let r: &String = &s; // Borrowing a reference to s
    // r.push_str(" World"); // This would cause an error, because r is a reference and cannot be modified
    println!("Borrowed: {}", r);

    let r2: &String = &s; // Multiple immutable references are allowed
    println!("Borrowed again: {}", r2);

    let r3: &mut String = &mut s; // Mutable references are allowed, but if we have a immutable reference being used after, will cause an error
    r3.push_str(" World"); // We can modify the value through the mutable reference
    println!("Modified: {}", r3);
}

// Slices
// Slices are a way to reference a contiguous sequence of elements in a collection
fn slice_example() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..4]; // Slicing the array to get a reference to a part of it
    println!("Slice: {:?}", slice);
}

// Strings
// Texts in Rust are encoded in UTF-8
// Rust has two different types of strings: String and &str
// String is a growable, heap-allocated data structure, while &str is a string slice
// String is mutable, while &str is immutable
fn string_example() {
    let s1: &str = "Olá mundo! 🍕";
    let s2: String = String::from("Olá mundo! 🍕");
    println!("s2: {}", s2);
    let s3: String = "Olá mundo! 🍕".to_string();
    println!("s3: {}", s3);
    let s4: &str = &s1[..];
    println!("s4: {}", s4);

    let mut foo: String = String::from("Foo");
    foo.push_str("bar");
    println!("foo: {}", foo);

    foo.replace_range(.., "Baz");
    println!("foo after replace: {}", foo);
}

// Concatenation
fn concatenation_example() {
    let s1: String = String::from("Olá");
    let s2: String = String::from("mundo!");
    let s3: String = s1 + " " + &s2; // Concatenating strings
    println!("s3: {}", s3);

    // format macro
    let s4: String = String::from("Hello");
    let s5: String = String::from("world");
    let s6: String = format!("{} {}", s4, s5);
    println!("s6: {}", s6);

    let s7: String = ["Hello", " ", "world"].concat();
    println!("s7: {}", s7);
}

// Indexing Strings
fn index_strings() {
    let s1: &str = "🍕🍕🍕🍕🍕";
    // let s2 = s1[0]; // This would cause an error, because Rust does not allow indexing into strings directly using a integer
    // let s2: &str = &s1[0..2]; // This is how you can get a slice of the string, however this would cause a panic because the 🍕 has a length of 4 bytes
    let s2: &str = &s1[0..4];
    println!("s2: {}", s2);
}

// Structs
// Structs are like Types/Interface in typescript
// Derive allow us to automatically implement traits for our struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_example() {
    let mut user: User = User {
        username: String::from("John Doe"),
        email: String::from("john.doe@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let name: String = user.username;
    println!("name: {}", name);
    println!("sign_in_count: {}", user.sign_in_count);
    println!("active: {}", user.active);
    user.email = String::from("john.doe.mutable@example.com");

    let user2: User = build_user(
        String::from("jane.doe@example.com"),
        String::from("Jane Doe"),
    );
    println!("user2: {:?}", user2);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);

    let user3: User = User { ..user2 };
    println!("user3: {:?}", user3);
    println!("sign_in_count: {}", user3.sign_in_count);
    println!("active: {}", user3.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
    }
}

// Tuple Structs are similar to structs, but they are similar to tuples in that they do not have named fields
fn tuple_struct() {
    struct Color(i32, i32, i32);
    let black: Color = Color(0, 0, 0);

    let r: i32 = black.0;
    let g: i32 = black.1;
    let b: i32 = black.2;
    println!("r: {}, g: {}, b: {}", r, g, b);
}

struct Rect {
    width: u32,
    height: u32,
}

fn calc_area_rect(dimenstions: &Rect) -> u32 {
    let rect: Rect = Rect {
        width: dimenstions.width,
        height: dimenstions.height,
    };

    println!("Area of rectangle: {}", rect.width * rect.height);
    rect.width * rect.height
}

// Methods
// Methods are functions that are associated with a struct
// They are defined inside the struct and have access to all the fields of the struct
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn calc_area_rect2(dimenstions: &Rect) -> u32 {
    let rect: Rect = Rect {
        width: dimenstions.width,
        height: dimenstions.height,
    };

    let rect1: Rect = Rect {
        width: 40,
        height: 60,
    };

    let rect2: Rect = Rect {
        width: 20,
        height: 30,
    };

    let rect3: Rect = Rect::square(10);

    println!("Area of rectangle using method: {}", rect.area());
    println!("Can rect hold rect1: {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2: {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Area of square: {}", rect3.area());
    rect.area()
}

// Methods, use the `self` keyword to access fields of the struct, while associated functions do not use `self`
// Associated functions are functions that are associated with a struct
// They are defined inside the `impl` block and do not have access to the fields of the struct
impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

// Enums
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_example() {
    let ip_v4: IpAddr = IpAddr {
        kind: IpAddrKind::V4(192, 168, 1, 1),
        address: String::from("192.168.1.1"),
    };
    let ip_v6: IpAddr = IpAddr {
        kind: IpAddrKind::V6("::1".to_string()),
        address: String::from("::1"),
    };

    println!("ip_v4: {:?}", ip_v4);
    println!("ip_v6: {:?}", ip_v6);

    // Use the fields of IpAddrKind::V4 to avoid dead_code warning
    if let IpAddrKind::V4(a, b, c, d) = ip_v4.kind {
        println!("IPv4 fields: {}.{}.{}.{}", a, b, c, d);
        println!("IPv4 address field: {}", ip_v4.address);
    }

    if let IpAddrKind::V6(addr) = ip_v6.kind {
        println!("IPv6 address: {}", addr);
        println!("IPv6 address field: {}", ip_v6.address);
    }
}

// Option Enum
// The Option enum is a powerful way to handle optional values in Rust
// It can either be Some(T) or None, where T is the type of the value
// This allows us to avoid null pointer exceptions and handle cases where a value may not be present
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn option_example() {
    let some_value: Option<i32> = Option::Some(5);
    let no_value: Option<i32> = Option::None;

    match some_value {
        Option::Some(v) => println!("Some value: {}", v),
        Option::None => println!("No value"),
    }

    match no_value {
        Option::Some(v) => println!("Some value: {}", v),
        Option::None => println!("No value"),
    }
}

// Matching on Option

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        }
        Coin::Nickel => {
            println!("This is a nickel");
            5
        }
        Coin::Dime => {
            println!("This is a dime");
            10
        }
        Coin::Quarter => {
            println!("This is a quarter");
            25
        }
    }
}

// IF LET
fn if_let_example() {
    let some_value = Some(5);

    // Match is more expensive than if let, so if we only care about one case, we can use if let
    match some_value {
        Some(5) => println!("Some value"),
        _ => println!("No value"),
    }

    if let Some(5) = some_value {
        println!("Some value");
    }
}

// Result
// enum Result<T, E> {
//     OK(T),
//     Err(E),
// }

// fn result_example() {
//     let result: Result<i32, String> = Result::OK(5);
//     let error: Result<i32, String> = Result::Err(String::from("An error occurred"));

//     match result {
//         Result::OK(v) => println!("Result is OK: {}", v),
//         Result::Err(e) => println!("Error: {}", e),
//     }

//     match error {
//         Result::OK(v) => println!("Result is OK: {}", v),
//         Result::Err(e) => println!("Error: {}", e),
//     }
// }

enum Operation {
    Add(i32, i32),
    Mul(i32, i32),
    Sub { first: i32, second: i32 },
    Div { divident: i32, divisor: i32 },
}

impl Operation {
    fn execute(self) -> Result<i32, String> {
        match self {
            Self::Add(a, b) => Ok(a + b),
            Self::Mul(a, b) => Ok(a * b),
            Self::Sub { first, second } => Ok(first - second),
            Self::Div { divident, divisor } => {
                if divisor == 0 {
                    Err(String::from("Can not divide by zero"))
                } else {
                    Ok(divident / divisor)
                }
            }
        }
    }
}

fn operation_example() {
    let user_input = Operation::Div {
        divident: 20,
        divisor: 0,
    };
    match user_input.execute() {
        Ok(res) => println!("Result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
}
