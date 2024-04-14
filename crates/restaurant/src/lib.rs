mod back_of_house;
mod front_of_house;

#[allow(dead_code)]
fn deliver_order() {}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
