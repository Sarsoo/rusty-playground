use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod collections;

fn main() {
    basics();
    compare();
    loops();
    vars();
    compound();

    println!("testing returns: {}", returning(2));
    strings();
    nullables();
}

fn basics() { 
    // ! indicates a macro not a func
    println!("Enter a number: ");

    // mut defines a mutable variable
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        // nice error handling wrapper on Result from read_line
        .expect("Failed to read line");
    
    // shadow previously defined variable
    // i32, u32, i64 (u = unsigned)
    let input: u32 = match input.trim() // std str trim (\n)
                                .parse() // infers what to parse to using type annotation
    {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("You entered: {}", input);
    
    // thread local random number gener
    let r_num = rand::thread_rng().gen_range(1, 101);
    println!("random number: {}", r_num);

    let vec = build_vector3(1, 2, 3);
    println!("Vector: {}/{}/{}", vec.x, vec.y, vec.z);
    let vec2 = update_x(vec);
    println!("Vector2: {:#?}", vec2)
}

fn compare() {
    let first = 5;
    let second = 6;

    match first.cmp(&second) {
        Ordering::Less => println!("Smaller"),
        Ordering::Greater => println!("Bigger"),
        Ordering::Equal => println!("Equal"),
    }
}

fn loops() {
    let var = 2;
    loop {
        println!("Looping!");
        
        if var == 2 {
            break;
        }
    }

    println!("Done with loop");

    let mut count = 0;
    let mut result = loop {
        count += 1;

        if count == 4 {
            break count *6;
        }
    };

    while result != 0 {
        result -= 1;
    }

    let iterables = [1, 2, 3, 4];
    for el in iterables.iter() {
        println!("iterable value: {}", el);
    }

    // 6 exclusive
    for num in (1..6).rev() {
        println!("ranged values: {}", num);
    }
}

fn vars() {
    // must be type annotated
    const X: i32 = 3_000_000;
    println!("const: {}", X);

    // suffix type on a scalar literal
    let _literal = 28u8;
    let _f = 2.0; // defaults to 64 bit
}

fn compound() {
    let tup = (500, 'a');
    let (_, second) = tup;
    println!("tuple: {}/{}", tup.0, second);

    let _arr: [u8; 4] = [1, 2, 3, 4];
    let repeater = [3; 5]; // stretches to 5 elements of 3
    println!("Retrieving from array: {}", repeater[3]);
}

fn returning(input: i32) -> i32 {
    let _stuff = 0;

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    input + 1 // no semicolon, return value
}

fn strings() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("Sliced: {}", &s[4..8]);
}

#[derive(Debug)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    fn dot(&self, other: &Vector3) -> i32 {
        self.x * other.x
        +
        self.y * other.y
        +
        self.z * other.z
    }
}

fn build_vector3(x: i32, y: i32, z: i32) -> Vector3 {
    Vector3 {
        x, y, z
    }
}

fn update_x(vec: Vector3) -> Vector3 {
    Vector3 {
        x: 5,
        ..vec
    }
}

enum ColourChannel {
    Red,
    Green,
    Blue
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn nullables() {
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}