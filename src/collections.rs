use std::collections::HashMap;

// tuples and arrays
// both stack-based
pub fn compound() {
    let tup = (500, 'a');
    let (_, second) = tup;
    println!("tuple: {}/{}", tup.0, second);

    // all of same type, [u8; 4] = 4 u8 types
    let _arr: [u8; 4] = [1, 2, 3, 4];
    let repeater = [3; 5]; // broadcasts 5 elements of 3
    println!("Retrieving from array: {}", repeater[3]);
}

pub fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    v.push(5);

    let mut v: Vec<_> = v.iter().map(|x| x + 1).collect();

    println!("{:?}", v);
    println!("2nd element is {}, {}", 
        &v[1], // will crash with index-out-of-bounds
        v.get(1).unwrap() // get an option to handle gracefully
    );

    for i in &mut v{
        *i += 1;
        println!("item: {}", i);
    }
}

pub fn hash() {
    let mut map = HashMap::new();

    map.insert("a".to_string(), 5);

    map.insert("b".to_string(), 6);
    map.insert("b".to_string(), 7); // overwrites

    // map.insert("c".to_string(), 10);
    map.entry("c".to_string()).or_insert(8);

    println!("{:?}", map);

    for (key, val) in map {
        println!("{}: {}", key, val);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}