extern crate yew;
extern crate yew_cat;

use yew::prelude::*;
use yew_cat::Model;

fn main() {
    println!("Hello, world!");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
