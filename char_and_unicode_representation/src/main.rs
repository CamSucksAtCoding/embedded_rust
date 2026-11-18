fn main() {
    /*
     * Using unicode to create a char variables in rust
     *
     * syntax: '\u{}'
     *
     *
     */
    let inifinity_symbol = '\u{221E}';
    println!(
        "Symbol: {inifinity_symbol}, Unicode Scalar Value: {}",
        inifinity_symbol as u32
    );
    let hope_youve_been_working_on_ur_chinese = '\u{00A5}';
    println!(
        "Shanghai Sharks Currency: {hope_youve_been_working_on_ur_chinese}, USV: {}",
        hope_youve_been_working_on_ur_chinese as u32
    );
}
