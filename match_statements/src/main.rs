fn main() {
    /*
     * Match Statements
     * Is used to perform a match against a value and execute different code based on which pattern matches the value.
     * if no pattern matches the value, the catch all pattern (_) is executed
     */

    let x = 69;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        69 => println!("Hehehe"),
        _ => println!("Where's Rick... Don't Care, Where's Rick."),
    }

    let point = (3, -1);
    match point {
        (_, y) if y < 0 => {
            println!("Y is less negative")
        }
        (0, 0) => {
            println!("Both equal zero")
        }
        _ => {
            println!("Aye nothing matches man")
        }
    }

    let random_array = [1, -3, -69, 420];
    for e in random_array {
        /*
         * What Im saying here is: Match the current element e if e is less than 0
         */
        match e {
            e if e < 0 => {
                println!("negative number found {e}");
            }
            _ => println!("No negative found"),
        }
    }
    //Teachers way to tackle problem
    match random_array {
        [n, _, _, _] if n < 0 => println!("Negative found at index 0: {n}"),
        [_, n, _, _] if n < 0 => println!("Negative found at index 1: {n}"),
        [_, _, n, _] if n < 0 => println!("Negative found at index 2: {n}"),
        [_, _, _, n] if n < 0 => println!("Negative found at index 3: {n}"),
        _ => println!("Your array is all positive numbers"),
    }
    //Teachers other way using matches!
    let mut invalid_array =
        matches!(random_array, [n, _, _, _] | [_, n, _, _] | [_, _, n, _] | [_, _, _, n] if n < 0);
    if invalid_array == true {
        invalid_array = false;
        println!("Is this array is valid? {invalid_array}");
    } else {
        invalid_array = true;
        println!("Is this array is valid? {invalid_array}");
    }
}
