pub fn variables() {

    // initialize a single variable
    let first_bunny: i32 = 2;

    // destructure a tuple
    let (second_bunny, first_carrot): (i32, i32) = (2, 3);

    // variable are immutable by default
    let third_bunnies: i32= 16;
    // Why would Rust have immutable variables by default?
    // safety concurrency and speed
    // data that never changes can be shared between multiple threads
    // without locks, so concurrency is improved ny immutability
    // the compiler can also do extra optimizations on data it knows own't change, so speed is improved by immutability

    let mut fourth_bunny = 20;
    println!("fourth_bunny: {}", fourth_bunny);
    fourth_bunny = 21;

    const WARP_FACTOR: f64 = 9.9;
    
    println!("{} {} {} {} {} {}", first_bunny, first_carrot, second_bunny, third_bunnies, fourth_bunny, WARP_FACTOR);
}