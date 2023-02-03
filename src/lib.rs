mod front_of_house {
    pub mod hosting {

        fn seat_at_table() {}

        pub fn add_to_wait_list() {}
    }

    pub mod serving {

        fn serve_order() {}

        pub fn take_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}

fn deliver_order() {}

fn eat_at_restaurant() {
    let appetizer_1 = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Bread");
    meal.toast = String::from("Yolo");

    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    front_of_house::serving::take_order();
}
