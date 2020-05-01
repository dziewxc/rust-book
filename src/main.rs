mod flowers;
use flowers::caring::watering;

//use std::io::{self, Write};
//use std::collections::*;

pub mod cleaning {
    //watering();
    pub mod kitchen {
        use crate::flowers::Type::Zamioculcas;

        fn dishes() { //private
            println!("dishes cleaned!")
        }

        pub fn kitchen_plants_watering() {
/*            let flower2 = crate::flowers::Flower { //this doesn't have access
                name: String::from("ZZ"),
                size:6
            };*/
            let mut flower2 = crate::flowers::create_flower(String::from("ZZ"), 6, Zamioculcas);
            println!("I have a new flower to water in the kitchen. Its name is {}", flower2.name);

            //editing name
            flower2.name = String::from("Zamioculcas");
            //editing private size is not possible
            //flower2.size = 2;
        }
    }
}

pub fn get_plant_list() {
    watering();
}

fn main() {
    watering();
    crate::cleaning::kitchen::kitchen_plants_watering();
}