extern crate wasm_learn;

use wasm_bindgen_test::*;
use wasm_learn::Universe;

#[cfg(test)]
pub fn input_space_ship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width_height(6, 6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width_height(6, 6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

// #[allow(unused_variables)]
// fn main() {
//
// }

#[wasm_bindgen_test]
pub fn test_trick() {
    let mut input_universe = input_space_ship();
    let expected_universe = expected_spaceship();

    input_universe.trick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

