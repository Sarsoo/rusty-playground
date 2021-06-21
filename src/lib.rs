//! Crate documentation

/// Print hello world to the screen
/// # Title
pub fn print_hello(){
    println!("Hello World!");
}

/// Return sum of input
/// 
/// # Examples
/// ```
/// let z = rustyplayground::add(1, 2);
/// 
/// assert_eq!(3, z)
/// ```
pub fn add(x: isize, y: isize) -> isize {
    x + y
}

pub mod new_mod {
    //! # Documented module
    //! 
    
    pub fn fun() {

    }
}