fn main() {
    /*
     * if let:
     * Is a shorthand for a match statement with just one pattern
     */
    let point = (0, 0);
    if let (0, 0) = point {
        println!("Nobody has scored yet");
    } else {
        println!("Something something about scoring");
    }
}
