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

fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    front_of_house::serving::take_order();
}
