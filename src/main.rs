#![allow(non_snake_case)]
#![allow(unused_parens)]
pub mod instructions;
pub mod compiler;

fn main() {

    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: u16 = 0;
    let mut programCounter: u8 = 1;
    let mut flag: bool = false;
    
    let _result = compiler::compileToFile();
    compiler::runCompiledFile(&mut memory);

    let mut counter: i32 = 0;
    while(memory[usize::from(programCounter)] != 0 && counter < 500) {
        instructions::executeInstruction(&mut programCounter, &mut accumulator, &mut flag, &mut memory);
        counter+=1;
    }

}