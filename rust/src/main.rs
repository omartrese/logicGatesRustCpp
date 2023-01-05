use std::io;

fn main() {
    jump_line(1);

    let mut one_gate = String::new();
    let mut two_gate = String::new();

    println!("Enter the first gate:");

    io::stdin()
        .read_line(&mut one_gate)
        .expect("failed to read input");

    let one_gate: u32 = one_gate.trim().parse().expect("ENTER A NUMBER");

    println!("The first GATE: {one_gate}");
    if one_gate > 1
    {
        println!("NO VALID GATE (one_gate)");
        return;
    }
    jump_line(1);

    println!("Enter the second gate:");
    
    io::stdin()
        .read_line(&mut two_gate)
        .expect("failed to read input");

    let two_gate: u32 = two_gate.trim().parse().expect("type a number between 0 and 1");

    println!("The second GATE: {two_gate}");

    if two_gate > 1
    {
        println!("NO VALID GATE (two_gate)");
        return;
    }

    jump_line(1);

    let and_gate: u32 = and_gate(one_gate, two_gate);
    let one_not_gate: u32 = not_gate(one_gate);
    let two_not_gate: u32 = not_gate(two_gate);
    let nand_gate: u32 = nand_gate(one_gate, two_gate);
    let or_gate: u32 = or_gate(one_gate, two_gate);
    let nor_gate: u32 = nor_gate(one_gate, two_gate);
    let xor_gate: u32 = xor_gate(one_gate, two_gate);
    let nxor_gate: u32 = nxor_gate(one_gate, two_gate);
    println!("AND GATE: {and_gate}");
    println!("NOT GATE (one): {one_not_gate}");
    println!("NOT GATE (two): {two_not_gate}");
    println!("NAND GATE: {nand_gate}");
    println!("OR GATE: {or_gate}");
    println!("NOR GATE: {nor_gate}");
    println!("XOR GATE: {xor_gate}");
    println!("NXOR GATE: {nxor_gate}");
}

fn and_gate(gate_one: u32, gate_two: u32) -> u32
{
    if gate_one == 1 && gate_two == 1
    {
        return 1;
    } else {
        return 0;
    }
}

fn not_gate(gate: u32) -> u32
{
    if gate == 1 {return 0} else {return 1};
}

fn nand_gate(gate_one: u32, gate_two: u32) -> u32
{
    return not_gate(and_gate(gate_one, gate_two));
}

fn or_gate(gate_one: u32, gate_two: u32) -> u32
{
    return nand_gate(not_gate(gate_one), not_gate(gate_two));
}

fn nor_gate(gate_one: u32, gate_two: u32) -> u32
{
    if or_gate(gate_one, gate_two) == 1 {return 0} else {return 1};  
}

fn xor_gate(gate_one: u32, gate_two: u32) -> u32
{
    return and_gate(or_gate(gate_one, gate_two), nand_gate(gate_one, gate_two));
}

fn nxor_gate(gate_one: u32, gate_two: u32) -> u32
{
    if xor_gate(gate_one, gate_two) == 1 {return 0} else {return 1};
}

fn jump_line(number_line: u32) //FUNCTION TO MAKE SPACE BETWEEN LINES
{
    let mut n: u32 = 0;

    while n < number_line
    {
        println!("");
        n += 1;
    }
}