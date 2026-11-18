fn main() {
    //Create a slice of a string
    //
    let name: String = String::from("Sigismund");
    let _blah: &str = "Blah blah blah"; // this is a prim type of str
    let slice_name: &str = &name[0..=4]; //This is a prim type of slice
    println!("My name is {name}, and I created a slice of my name {slice_name}");

    /*
     * How is PT slice and PT str different:
     * str represents a sequence of UTF-8 characters that are immutable,
     * and is accessed using a reference &str, which allows safe read only operations
     *
     * slice is a dynamic view of a contiguous sequence of elements, it can be accessed as &[T]
     * for immutable access or &mut[T] for mutable access
     */
}
