/*
 * Option Enums: encapsulates the possibility of either having a value or not having a value. It
 * represents the concept of an optional value in Rust, can be either Some(value) or None
 *
 * Important: Rust encourages devs to handle nullability explicitly and forces us to account for the possibility
 * of absence of values in code, this prevents null reference errors and encourages safer and more robust software
 */
//Less efficient way
fn find_value(array: &[i32], find_x: &i32) -> i32 {
    let mut index = 0;
    for e in array {
        if e == find_x {
            println!("Value {find_x} was found in the array at index {index}");
            return index;
        }
        index += 1;
    }
    println!("Value not found in the array");
    -1
}
fn find_value_bttr(array: &[i32], find_x: &i32) -> Option<i32> {
    for (index, value) in array.iter().enumerate() {
        if value == find_x {
            println!("Value {find_x} found in array at {index}");
            return Option::Some(index as i32);
        }
    }
    println!("Value not found");
    None
}
fn main() {
    /* initial testing
    //Option creation
    let _v_1 = Option::Some(20); //Basic creation
    let _v_2: Option<String> = Some("Cameron".to_string()); //Explicit Type creation
    let _v_3: Option<i32> = None; //Explicit type creation with value set to None
    let _v_4 = Option::<i32>::None; //Short hand

    let testing_option = Option::<String>::Some("ZL1 Camaro".to_string());
    println!("Printing an Option Enum: {testing_option:?}");
    */
    let my_array: [i32; 4] = [69, 420, 67, 25];
    let find = 4201;
    find_value(&my_array, &find);
    find_value_bttr(&my_array, &find);
}
