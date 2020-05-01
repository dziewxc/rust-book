pub struct Flower { //pub struct won't make all properties automatically pub
pub name: String,
    size: u8,
    pub flower_type: Type
}

pub enum Type { //pub enum will make all values automatically pub
Zamioculcas,
    Sansevieria
}

pub fn create_flower(name: String, size: u8, flower_type: Type) -> Flower {
    Flower {
        name,
        size,
        flower_type
    }
}

pub mod caring {
    pub fn watering() {
        let flower1 = super::Flower { //child has access
            name: String::from("ZZ"),
            size: 3,
            flower_type: crate::flowers::Type::Sansevieria
        };
        //super::super::get_plant_list();
        println!("plant watered!")
    }
}