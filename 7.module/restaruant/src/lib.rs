mod back_of_house;
mod front_of_house;

pub use crate::back_of_house::Appetizer;
pub use crate::back_of_house::Breakfast::Breakfast;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd link {}  toast pease", meal.toast);
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}
