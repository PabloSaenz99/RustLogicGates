use gates::print_gate;

use crate::{gates::{calculate_output, GateTypes}, logic_gate_base::LogicGate};

mod logic_gate_base;
pub mod gates;


fn main() {
    let mut o= LogicGate::new(GateTypes::OR);
    // o.set_right_input(true);
    calculate_output(&mut o);
    let mut a= LogicGate::new(GateTypes::AND);
    a.set_left_input(true);
    a.set_right_input(true);
    calculate_output(&mut a);

    print_gate(&o);
    print_gate(&a);

    println!("-------------------------------");
    o.set_left_input_connection( Some(Box::new(a)));
    print_gate(&o);
}
/*
fn main() {
    println!("--------------OR---------------");
    let mut o= LogicGate::new(GATES::OR);
    calculate_output(&mut o);
    println!("{}", o.get_string()[0]);
    println!("{}", o.get_string()[1]);
    println!("{}", o.get_string()[2]);
    println!("----CALC----");
    o.set_right_input(true);
    calculate_output(&mut o);
    println!("{}", o.get_string()[0]);
    println!("{}", o.get_string()[1]);
    println!("{}", o.get_string()[2]);
    println!("----CALC----");
    o.set_left_input(true);
    calculate_output(&mut o);
    println!("{}", o.get_string()[0]);
    println!("{}", o.get_string()[1]);
    println!("{}", o.get_string()[2]);
    println!("----CALC----");

    println!("--------------AND---------------");
    let mut a= LogicGate::new(GATES::AND);
    calculate_output(&mut a);
    println!("{}", a.get_string()[0]);
    println!("{}", a.get_string()[1]);
    println!("{}", a.get_string()[2]);
    println!("----CALC----");
    a.set_right_input(true);
    calculate_output(&mut a);
    println!("{}", a.get_string()[0]);
    println!("{}", a.get_string()[1]);
    println!("{}", a.get_string()[2]);
    println!("----CALC----");
    a.set_left_input(true);
    calculate_output(&mut a);
    println!("{}", a.get_string()[0]);
    println!("{}", a.get_string()[1]);
    println!("{}", a.get_string()[2]);
    println!("----CALC----");
} */