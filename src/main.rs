mod variable;
mod scope;
mod memory_safety;
pub fn main() {
    println!("Hello, world!");
    variable::variables();
    scope::scope();
    memory_safety::memory_safety();
}
