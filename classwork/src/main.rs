mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::derived::user_defined;
use how_you_hold_data_for_operations::primitives::scalar;
use how_you_hold_data_for_operations::primitives::compound;

fn main() {
    scalar::run();
    compound::run();
    user_defined::run();
}
