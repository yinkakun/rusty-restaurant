use std::fmt::Result;
use std::io::Result as IoResult;

use std::collections::{BinaryHeap, HashMap};

mod back_of_house;
mod front_of_house;

fn deliver_order() {}

use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    let appetizer_1 = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Bread");
    meal.toast = String::from("Yolo");

    // absolute path
    hosting::add_to_wait_list();

    // relative path
    front_of_house::serving::take_order();
}

mod customer {

    fn eat_at_restaurant() {
        super::front_of_house::hosting::add_to_wait_list()
    }
}
