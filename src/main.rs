#![allow(non_snake_case)]
#![allow(unused_parens)]
pub mod instructions;
pub mod compiler;

fn helpFunction() {
    println!("-------------------------------------------------");
    println!("help - returns a list of functions");
    println!("compile - compiles the file down to .lmc format");
    println!("run - runs the compiled .lmc file");
    println!("-------------------------------------------------");
}

fn compileFunction() {
    let _result = compiler::compileToFile();
}

fn runFunction() {
    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: u16 = 0;
    let mut programCounter: u8 = 1;
    let mut flag: bool = false;

    compiler::compiledFileToMem(&mut memory);

    println!("---------------------------");

    let mut counter: i32 = 0;
    while(memory[usize::from(programCounter)] != 0 && counter < 500) {
        instructions::executeInstruction(&mut programCounter, &mut accumulator, &mut flag, &mut memory);
        counter+=1;
    }
    println!("---------------------------");
}

fn main() {

    let mut consoleInput: String = String::new();

    while(consoleInput != "q")
    {
        consoleInput = String::new();
        std::io::stdin().read_line(&mut consoleInput).expect("error: unable to read user input");
        consoleInput = consoleInput.replace("\r\n", "");
  
        match consoleInput.as_str() {
           "help" => helpFunction(),
           "compile" => compileFunction(),
           "run" => runFunction(),
           _ => continue,
        }
    }
}