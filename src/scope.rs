// variable begins where it is created and extends to the end of the block.
pub fn scope () {
    // can also shadow variables in the same scope
    // let mut x =5; // x is mutable
    let x = "xy"; // x is now immutable
    // data transformation pipelines that discard intermediate representations
    {
        let y = "10";
        println!("x: {}, y: {}", x, y);
    }
    println!("x: {}", x);
}