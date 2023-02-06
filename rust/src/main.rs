use std::io;


fn main() {
    jump_line(1);

    let mut gates_number_one = String::new();
    let mut gates_number_two = String::new();
    
    let one_gate: bool;
    let two_gate: bool;    

    println!("Enter the first gate:");

    io::stdin()
        .read_line(&mut gates_number_one)
        .expect("failed to read input");

    one_gate = gates_verification( gates_number_one.trim().parse().expect("ENTER A NUMBER"));

    println!("The first GATE: {one_gate}");
 
    jump_line(1);

    println!("Enter the second gate:");
    
    io::stdin()
        .read_line(&mut gates_number_two)
        .expect("failed to read input");

    two_gate = gates_verification(gates_number_two.trim().parse().expect("type a number between 0 and 1"));

    println!("The second GATE: {two_gate}");

    jump_line(1);

    let and_gate: bool = and_gate(one_gate, two_gate);
    let one_not_gate: bool = not_gate(one_gate);
    let two_not_gate: bool = not_gate(two_gate);
    let nand_gate: bool = nand_gate(one_gate, two_gate);
    let or_gate: bool = or_gate(one_gate, two_gate);
    let nor_gate: bool = nor_gate(one_gate, two_gate);
    let xor_gate: bool = xor_gate(one_gate, two_gate);
    let nxor_gate: bool = nxor_gate(one_gate, two_gate);
    println!("AND GATE: {and_gate}");
    println!("NOT GATE (one): {one_not_gate}");
    println!("NOT GATE (two): {two_not_gate}");
    println!("NAND GATE: {nand_gate}");
    println!("OR GATE: {or_gate}");
    println!("NOR GATE: {nor_gate}");
    println!("XOR GATE: {xor_gate}");
    println!("NXOR GATE: {nxor_gate}");
}

fn and_gate(gate_one: bool, gate_two: bool) -> bool
{
    if gate_one && gate_two
    {
        return true;
    } else {
        return false;
    }
}

fn not_gate(gate: bool) -> bool
{
    return !gate;
}

fn nand_gate(gate_one: bool, gate_two: bool) -> bool
{
    return not_gate(and_gate(gate_one, gate_two));
}

fn or_gate(gate_one: bool, gate_two: bool) -> bool
{
    return nand_gate(not_gate(gate_one), not_gate(gate_two));
}

fn nor_gate(gate_one: bool, gate_two: bool) -> bool
{
    return not_gate(or_gate(gate_one, gate_two));
}

fn xor_gate(gate_one: bool, gate_two: bool) -> bool
{
    return and_gate(or_gate(gate_one, gate_two), nand_gate(gate_one, gate_two));
}

fn nxor_gate(gate_one: bool, gate_two: bool) -> bool
{
    return not_gate(xor_gate(gate_one, gate_two));
}

fn jump_line(number_line: i32) //FUNCTION TO MAKE SPACE BETWEEN LINES
{
    let mut n= 0;

    while n < number_line
    {
        println!("");
        n += 1;
    }
}

fn gates_verification(gate: u32) -> bool
{
    match gate
    {
        0 => return false,
        1 => return true,
        _ => {
        println!("INSERT A NUMBER!!! (0 or 1)");
            std::process::abort()
        },
    }
}