fn main() {
    /*
     * Storing ascii code
     *
     * Use the primitive type u8
     *
     * u8 is an unsigned 8 bit integer, and can store values 0 - 255 inclusive
     *
     * Use the 'b' prefix if you want to store the ascii codes using ascii characters in the code
     *
     *
     */

    let ascii_code_of_plus_value = b'+';
    let ascii_code_of_plus_symbol = b'+' as char;
    println!(
        "Ascii code of plus symbol: {ascii_code_of_plus_value} Ascii symbol of plus as a char: {ascii_code_of_plus_symbol}"
    );
}
