fn main() {
    /*
     * References:
     * a reference(&T or &mut T) is a lightweight pointer that lets you access a value without taking ownership of it.
     * Unlike raw pointers, references are checked by the compiler for lifetimes and borrowing rules, guaranteeing they
     * are always valid and preventing data races or dangling pointers
     *
     * Important: References can never be null or dangling
     *
     * Note: Rust has manual and automatic dereferencing, thank GOD for this
     *
     */

    let value: i64 = 42;
    //create a reference to value
    let ref_of_value = &value;

    //manual dereference
    println!("This was an automatic dereference: {}", *ref_of_value);
    //automatic dereference
    println!("This was an automatic dereference: {ref_of_value}");
    //print the memory address
    println!("Memory Address: {ref_of_value:p}");
}
