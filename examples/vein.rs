//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Checkerboard, Vein};

mod utils;

fn main() {
    let checker = Checkerboard::new(0);
    let vein = Vein::new(10, 4.0);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(vein)
            .set_x_bounds(-100.0, 100.0)
            .set_y_bounds(-100.0, 100.0)
            .build(),
        "vein.png",
    );
}
