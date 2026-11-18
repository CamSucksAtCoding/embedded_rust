fn main() {
    /*
     * Arrays
     *
     * Syntax: let variable_name : [type; size] = [element1, element2, etc];
     *
     * printing an array
     *println("{array:?}");
     *
     * Creating an array that has the same values stored  in it repeating
     * let repeating_array: [i32; 12] = [0; 12];
     *                                value|repeat
     *
     * array indexing
     * let element = array[3];
     */
    let my_array: [&str; 4] = ["Robot Gorillaman", "Peter Turbo", "Sangy", "Lemon Rust"];
    println!("{my_array:?}");
    let best_primarch = my_array[2];
    println!("Best Primarch: {best_primarch}");
    //iterate over the array
    for e in my_array {
        println!("Primarch Name: {e}");
    }

    let repeating_array: [i32; 12] = [0; 12];
    println!("{repeating_array:?}");
}
