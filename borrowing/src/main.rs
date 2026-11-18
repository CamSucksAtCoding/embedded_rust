fn main() {
    let num1 = 50; //even if this was mut I still couldnt change it with my reference bc its an imm borrow

    //I am borrowing the data from num1 to read it in ref_num1, I can't directly change num1 from the reference
    let _ref_num1 = &num1; //immutable borrow

    let mut num2 = 10000000;
    let ref_num2 = &mut num2;

    //modify num2
    *ref_num2 = 69;
    println!("Value of num2: {ref_num2}");
}
