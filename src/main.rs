#![allow(non_snake_case)]
#![allow(unused_parens)]
pub mod instructions;
pub mod compiler;

fn io(adr: &u8, acc: &mut u16) {
    match *adr{
        1=>instructions::inp(acc),
        2=>instructions::out(acc),
        _=>println!("error : invalid address"),
    }
}

fn executeInstruction(pc: &mut u8, acc: &mut u16, flag: &mut bool, mem: &mut Vec<u16>) {

    let index: u8 = *pc;

    let instruction: u8 = (mem[usize::from(index)]/100) as u8;
    let address: u8 = (mem[usize::from(index)]%100) as u8;
    let mut value: &mut u16 = &mut mem[usize::from(address)];

    match instruction{
        1=>instructions::add(acc, flag, value),
        2=>instructions::sub(acc, flag, value),
        3=>instructions::store(acc, &mut value),
        5=>instructions::load(acc, flag,value),
        6=>instructions::branch(pc, &address),
        7=>instructions::branchZero(pc, &address, acc, flag),
        8=>instructions::branchPositive(pc, &address, flag),
        9=>io(&address, acc),
        _=>println!("error : invalid instruction"),
    }
    *pc += 1;
}

fn main() {

    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: u16 = 0;
    let mut programCounter: u8 = 0;
    let mut flag: bool = false;
    
    let _result = compiler::compileToFile();

    while(memory[usize::from(programCounter)] != 0) {
        executeInstruction(&mut programCounter, &mut accumulator, &mut flag, &mut memory);
    }

}