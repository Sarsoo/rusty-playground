// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::{executor::block_on, Future};

async fn async_return() -> u8 { 
    5
}

fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = async_return().await;
        x + 5
    }
}

async fn async_thing() { 
    println!("an async thing");
}

async fn hello_world() { 
    println!("hello, world!");

    let f1 = async_thing();
    let f2 = async_thing();

    futures::join!(f1, f2);
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}


#[cfg(test)]
// #[warn(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }

}