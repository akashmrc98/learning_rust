// #[path = "./topics/1_intro.rs"] mod intro;
// #[path = "./topics/2_vars.rs"] mod vars;
// #[path = "./topics/3_data_types.rs"] mod data_types;
// #[path = "./topics/4_functions.rs"] mod functions;
// #[path = "./topics/5_control_flow.rs"]
// mod control_flow;
#[path = "./topics/6_owner_ship.rs"]
mod owner_ship;

fn main() {
    // intro::learn()
    // vars::learn();
    // data_types::learn();
    // functions::learn();
    // control_flow::learn();
    owner_ship::learn();
}
