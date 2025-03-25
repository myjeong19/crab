// rust guarantees memory safety at compile time.
// as apart of that, variables must be initialized before.

pub fn memory_safety () {
    // this code won't work because enigma has been declared but not initialized to value before we try to
    // let enigma;
    // println!("enigma: {}", enigma);

    // definite initialization
    // Rust requires variables to be fully initialized before use.
    // if a variable is only initialized in one branch (if without else), 
    // Rust cannot guarantee its initialization and throws an error.
    // if both if and else initialize the variable, Rust ensures it is always assigned, so no error occurs.
    // this strict rule enhances Rust’s memory safety by preventing the use of uninitialized memory. 
    // let enigma;
    // if true {
    //     enigma = 1;
    // }
    // println!("enigma: {}", enigma); // Error

    let enigma;
    if true {
        enigma = 42
    } else {
        enigma = 100;
    }
    println!("enigma: {}", enigma);
}

