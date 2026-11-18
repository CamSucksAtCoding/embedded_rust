fn main() {
    /*
     * Slices:
     * A slice is a borrowed portion of contiguous data, like an array or a string.
     *
     * Its true benefit, lies in its ability to let you work with a portion of data efficiently without ownership, copying,
     * or allocation
     */

    let my_array: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &my_array[0..3]; //grabs index 0 to 3
    println!("My slice: {slice:?}");
    let slice2 = &my_array[0..=3]; //adding = sign makes the index inclusive
    //or print them out individually
    for e in slice {
        println!("My slice: {e}");
    }

    for e in slice2 {
        println!("My slice2: {e}");
    }

    let slice3 = &my_array[2..]; //prints everything after index 2

    for e in slice3 {
        println!("My slice3: {e}");
    }

    let slice4 = &my_array[..]; //prints everything in the array

    for e in slice4 {
        println!("My slice4: {e}");
    }
}
