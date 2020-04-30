use std::io;

struct Shorts {
    color: String,
    brand: String,
    rating: u8,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let mut shorts1 = Shorts {
        color: String::from("blue"),
        brand: String::from("C&A"),
        rating: 1,
    };

    let mut user_rating = String::new();

    io::stdin()
        .read_line(&mut user_rating)
        .expect("wrong input");

    shorts1.rating = user_rating.trim().parse::<u8>().unwrap();

    println!("rating of the shorts: {}", shorts1.rating);

    let new_shorts = build_some_shorts(String::from("red"), String::from("H&M"));

    println!("New shorts have color: {} and brand: {}", new_shorts.color, new_shorts.brand);

    let shorts3 = Shorts {
        color: String::from("yellow"),
        ..new_shorts //struct update syntax
    };

    println!("Shorts 3 inherit brand and rate from shorts 2: brand - {}", shorts3.brand);

    //tuple structs

    struct ShirtColor(i32, i32, i32);

    struct Shirt {
        shirt_color: ShirtColor,
        shirt_brand: String,
    }

    let shirt = Shirt {
        shirt_color: ShirtColor(1, 1, 1),
        shirt_brand: String::from("Mohito"),
    };

    println!("this is my new shirt, its color is: {}", shirt.shirt_color.0);

    /*    struct Shoe {
            size: u8,
            brand: &str //missing lifetime specifier!
        }*/

    let rect = rectangle_project();
}

fn build_some_shorts(color: String, brand: String) -> Shorts {
    Shorts {
        color, //field init shorthand
        brand,
        rating: 0,
    }
}

fn rectangle_project() {
    let width1 = 30;
    let height1 = 60;

    let area = calculate_area(width1, height1);

    println!("are of rectangle: {}", area);

    //rectangle project with tuples:
    let rect2 = (20,30);
    let area2 = calculate_are_from_tuple(rect2);

    println!("tuple rect area: {}", area2);

    //rectangle project from struct

    let rect3 = Rectangle {
        height: 400,
        width: 500
    };

    let area3 = calculate_area_from_struct(&rect3);

    println!("area from struct: {}", area3);

    //debug print format, needs debug derive {:?}
    println!("take a look at this weird struct right here: {:?}", rect3);
    println!("a little prettier print: {:#?}", rect3)
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_are_from_tuple(tuple_rectangle: (u32, u32)) -> u32 {
    tuple_rectangle.0 * tuple_rectangle.1
}

fn calculate_area_from_struct(r: &Rectangle) -> u32 { //we prefer immutable borrow here
    r.height * r.width
}