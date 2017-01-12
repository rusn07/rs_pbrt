extern crate pbrt;

use pbrt::{BoxFilter, Float, Vector2f};

fn main() {
    // see box.cpp CreateBoxFilter()
    let xw: Float = 0.5;
    let yw: Float = 0.5;
    let box_filter = BoxFilter {
        radius: Vector2f { x: xw, y: yw },
        inv_radius: Vector2f {
            x: 1.0 / xw,
            y: 1.0 / yw,
        },
    };

    println!("box_filter = {:?}", box_filter);
}