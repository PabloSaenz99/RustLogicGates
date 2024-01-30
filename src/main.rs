use logic_gate_base::LogicGateOperations;

use crate::{and_gate::AND, or_gate::OR};

mod logic_gate_base;
mod or_gate;
pub mod and_gate;

fn main() {
    println!("--------------OR---------------");
    let mut o= OR::new();
    o.calculate_output();
    println!("{}", o.data.get_string()[0]);
    println!("{}", o.data.get_string()[1]);
    println!("{}", o.data.get_string()[2]);
    println!("----CALC----");
    o.data.set_right_input(true);
    o.calculate_output();
    println!("{}", o.data.get_string()[0]);
    println!("{}", o.data.get_string()[1]);
    println!("{}", o.data.get_string()[2]);
    println!("----CALC----");
    o.data.set_left_input(true);
    o.calculate_output();
    println!("{}", o.data.get_string()[0]);
    println!("{}", o.data.get_string()[1]);
    println!("{}", o.data.get_string()[2]);
    println!("----CALC----");

    println!("--------------AND---------------");
    let mut a= AND::new();
    a.calculate_output();
    println!("{}", a.data.get_string()[0]);
    println!("{}", a.data.get_string()[1]);
    println!("{}", a.data.get_string()[2]);
    println!("----CALC----");
    a.data.set_right_input(true);
    a.calculate_output();
    println!("{}", a.data.get_string()[0]);
    println!("{}", a.data.get_string()[1]);
    println!("{}", a.data.get_string()[2]);
    println!("----CALC----");
    a.data.set_left_input(true);
    a.calculate_output();
    println!("{}", a.data.get_string()[0]);
    println!("{}", a.data.get_string()[1]);
    println!("{}", a.data.get_string()[2]);
    println!("----CALC----");
}