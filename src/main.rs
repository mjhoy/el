extern crate el;

use el::*;

fn main() {
    startup(Some("config/log.yml"));
    intro();
    teardown();
}

