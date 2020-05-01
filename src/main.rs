use crate::Gender::Women;
use crate::Description::Unicorn;

#[derive(Debug)]
enum IpVersion {
    V4,
    V6
}

enum Message { //enum
    Move { x: i32, y: i32 }, // struct
    Write(String), //one string
    ChangeColor(i32, i32, i32), //3 int
}
//is like:

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
struct IpAddress {
    version: IpVersion,
    address: String
}

#[derive(Debug)]
enum IpAddr { //??o.O
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Description {
    NonBinary,
    Unicorn
}

enum Gender {
    Men,
    Women,
    Other(Description)
}

struct Person {
    name: String,
    gender: Gender
}

fn main() {
    let ipv4 = IpVersion::V4;
    route(&ipv4);

    let home = IpAddress {
        version: ipv4,
        address: String::from("127.1")
    };
    println!("Home address: {:#?}", home);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Loopback: {:#?}", loopback);

    let y = Option::Some(1);
    let x: Option<i8> = Option::None; //we need to add type
    //Option<i8> is not the same type as i8
    println!("some in Rust be like: {:#?}", y);
    println!("null in Rust be like: {:#?}", x);

    //let sum0 = y + 5; //implementation of Add missing
    //let sum = y + x; // always need to convert Option before using as real type

    let girly_girl = Person {
        name: String::from("Anna"),
        gender: Women
    };

    let unicorny = Person {
        name: String::from("Olaf"),
        gender: Gender::Other(Unicorn)
    };

    let likes_anna = peter_likes(girly_girl.gender);
    let likes_olaf = peter_likes(unicorny.gender);

    println!("Does Peter likes Anna? {}", likes_anna);
    println!("Does Peter likes Olaf? {}", likes_olaf);

    let f = Option::Some(5);
    let six = plus_one(f);
    println!("Six: {:?}", six);

    println!("6 is 5? {}", is_five(6));

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("its 3");
    }
}

fn plus_one(number: Option<u8>) -> Option<u8> {
    match number { //Matches in Rust are exhaustive
        None => None,
        Some(i)=> Some(i + 1)
    }
}

fn is_five(num: u8) -> bool {
    match num {
        5 => true,
        _ => false //matches any value
    }
}

fn peter_likes(gender: Gender) -> bool {
    match gender {
        Gender::Women => true,
        Gender::Men => true,
        Gender::Other(description) => {
            println!("Sorry, Peter doesn't like {:#?}", description);
            false
        }
    }
}

fn route(version: &IpVersion) {
    println!("you are using version: {:?}", version)
}