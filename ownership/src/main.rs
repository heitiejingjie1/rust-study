mod owner_ship;

use crate::owner_ship::{
    owner_ship::owner_display, reference::reference_display, slice::slice_display,
};

fn main() {
    owner_display();
    reference_display();
    slice_display();
}
