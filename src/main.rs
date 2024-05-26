#![allow(non_snake_case)]

fn executeInstruction(pc: &mut u8, acc: &mut u16, flag: &mut bool, mem: &mut Vec<u16>) {

    let index: u8 = *pc;

    let instruction: u8 = (mem[usize::from(index)]/100) as u8;
    let address: u8 = (mem[usize::from(index)]%100) as u8;

    
}

fn main() {

    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: u16 = 0;
    let mut programCounter: u8 = 0;
    let mut flag: bool = false;

    println!("Size : {}", memory[99]);
}
