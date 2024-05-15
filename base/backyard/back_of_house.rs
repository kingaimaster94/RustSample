mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant () {
        hosting::add_to_wait_list();
        // // Absolute path
        // crate::front_of_house::hosting::add_to_wait_list();
        // // Relative path
        // front_of_house::hosting::add_to_wait_list();
    
        //Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast == String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // The next line won't compile if we uncomment it; we're not allowed
        // To see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    
        let order1 = back_of_house::Appetizer::Soap;
        let order2 = back_of_house::Appetizer::Salad;
    }    
}

fn deliver_order() {}

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
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}