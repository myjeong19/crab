fn main() {
    println!("Hello, world!");

    // initialize a single variable
    let firstBunny: i32 = 2;

    // destructure a tuple
    let (secondBunny, firstBunny): (i32, i32) = (2, 3);

    // variable are immutable by default
    let third_bunnies= 16;
    // Why would Rust have immutable variables by default?
    // safety concurrency and speed
    // data that never changes can be shared between multiple threads
    // without locks, so concurrency is improved ny immutability
    // the compiler can also do extra optimizations on data it knows own't change, so speed is improved by immutability

    let mut fourthBunny = 20;
    fourthBunny = 21;

    const WARP_FACTOR: f64 = 9.9;
    

}
