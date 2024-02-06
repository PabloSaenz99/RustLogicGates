use controller::Controller;

use crate::{gates::GateTypes, input_utils::show_options};

mod logic_gate_base;
pub mod gates;
pub mod controller;
mod input_utils;

fn main() {
	let ctrl = Controller::new();
	ctrl.borrow_mut().add_new_gate(GateTypes::AND);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);

	println!("-----------");
	ctrl.borrow().print_tree();

	loop {
		show_options();
		ctrl.borrow_mut().parse_options();
	}
}