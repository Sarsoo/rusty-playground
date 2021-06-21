
// indireaction of box lets list be a fixed size
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_ptr() {
    let b = Box::new(5); // implements deref and drop
    println!("b = {}", b);
}