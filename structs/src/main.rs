/**
 * Structs:
 * Are custom data types that allow you to store related information, basically a class in more general terms
 *
 * Syntax: you will have fields in the struct that are followed by a colon and then the data type or value, all fields are
 *          are seperated by commas
 *
 * Format:
 *      struct NameInPascalCase {
 *          data_field: data type,
 *          data_field: data type,
 *      }
 */
struct Astartes {
    chapter: String,
    loyalist: bool,
    chapter_size: i32,
}
#[derive(Debug)]
struct Car {
    model: String,
    brand: String,
    price: f64,
    year: i32,
}
fn main() {
    //Create an instance of the Named Struct
    let blood_ravens = Astartes {
        chapter: String::from("Blood Ravens"),
        loyalist: true,
        chapter_size: 1000,
    };
    //Use . to print the values of each field
    println!("Chapter: {}", blood_ravens.chapter);
    println!("Loyalist Chapter: {}", blood_ravens.loyalist);
    println!("Chapter Size: {}", blood_ravens.chapter_size);

    let model = String::from("Shelby GT350R");
    let brand = String::from("Ford");
    let price = 60_000.00;
    let year = 2018;

    let shelby_gt350r = Car {
        model: model,
        brand: brand,
        price: price,
        year: year,
    };
    println!(
        "One of my dream cars is the {} {} {} which cost around ${} ",
        shelby_gt350r.year, shelby_gt350r.brand, shelby_gt350r.model, shelby_gt350r.price
    );
    println!("{shelby_gt350r:?}");
}
