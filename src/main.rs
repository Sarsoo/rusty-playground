use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use rand::Rng;
// use rustyplayground;

mod iterate;
mod macros;
mod structs;
mod collections;
mod threads;

fn main() {
    // rustyplayground::print_hello();
    // basics();
    // compare();
    // loops();
    // vars();
    // collections::compound();
    // collections::vectors();
    collections::hash();
    threads::start_threads();
    threads::thread_channel();

    // println!("testing returns: {}", returning(2));
    // strings();
    // nullables();

    nested_error_matching();
}

fn basics() { 

    println!("min: {}", find_min!(5, 6));

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

    let vec = structs::build_vector3(1, 2, 3);
    println!("Vector: {}/{}/{}", vec.x, vec.y, vec.z);
    let vec2 = structs::update_x(vec);
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

fn returning(input: i32) -> i32 {
    let _stuff = 0;

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    input + 1 // no semicolon, return value
}

fn strings() {
    let mut s = String::from("hello");
    let s2 = "world!".to_string();
    let s3 = String::from("hello");

    // cannot [] index strings due to utf-8 encoding

    s.push_str(", world!");
    println!("Sliced: {}", &s[4..8]); // slice
    println!("Appended: {}", s3 + " 2"); // takes ownership of s3
    println!("{}", 
        format!("{} {}", s, s2)
    ); 

    let s4 = "abcd".to_string();
    for i in s4.chars() {
        println!("{}", i);
    }

    for i in s4.bytes() {
        println!("{}", i);
    }
}

fn nullables() {
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}

fn nested_error_matching() {
    let f = File::open("hello.txt");

    // .unwrap() get inner value or panic
    // .expect() as above but with custom error message

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn propagate_err() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagate_err_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // continue if ok, return error if not, convert to return type if required
    Ok(s)
}

fn propagate_err_operator_shorter() -> Result<String, io::Error> {
    
    // can also be done with fs::read_to_string()

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // continue if ok, return error if not, convert to return type if required
    Ok(s)
}


#[cfg(test)]
// #[warn(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }

    #[ignore]
    #[test]
    #[should_panic(expected = "index out of bounds")] // check that panic message is substring of actual
    fn test_panic() {
        let arr = vec![1, 2, 3];
        arr[4];
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("Error".to_string())
        }
    }
}