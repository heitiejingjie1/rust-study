mod ownership;
mod reference_borrowing;
mod slice;

use ownership::owner;
use reference_borrowing::reference;
use slice::slice_demo::display;

use crate::slice::slice_demo;

fn main() {
    // owner::display();
    // reference::display();
    slice_demo::display();
}
