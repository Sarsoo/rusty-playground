
// declare lifetime after function name like generic type
// both parameters have same lifetime and must be alive when returned
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// doesn't change lifetimes of parameters, just annotates enough
// that the compiler can reject stuff that won't work

// structs can have reference members
// annotate with lifetimes to ensure struct doesn't outlive reference 
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// generic types, trait bounds and lifetimes
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}