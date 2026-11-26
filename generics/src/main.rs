use std::cmp;

/*
 * Generics: Allow you to write functions or types that can work with any type that meets certain
 *          requirements rather than being tied to a specific type.
 *
 */
//Trait Bounds: State what types that can be used with this generic function
fn find_max_element<T>(input: &[T]) -> Option<T>
where
    T: cmp::PartialOrd + Copy,
{
    let mut max = input[0];
    if input.is_empty() {
        return None;
    }

    for &elements in input {
        if elements > max {
            max = elements
        }
    }
    Some(max)
}
fn main() {
    let array_int = [1, 2, 3, 3, 8, 99, 56, 10000];
    let array_float = [6.009, 2.67, -4.89, 3.0, 8.8, 123.99, 9.56, 1.0000];
    println!("Max Element: {:?}", find_max_element(&array_int));
    println!("Max Element: {:?}", find_max_element(&array_float));
}
