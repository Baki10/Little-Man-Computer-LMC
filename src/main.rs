#![allow(non_snake_case)]
#![allow(unused_parens)]


pub mod instructions;
pub mod compiler;

fn helpFunction() {
    println!("---------------------------------------------------------------");
    println!("<> help - returns a list of functions");
    println!("<> compile <file path> - compiles the file down to .lmc format");
    println!("<> run <file path> - runs the compiled .lmc file");
    println!("<> q - quits the program");
    println!("---------------------------------------------------------------");
}

fn compileFunction(arg: &str) {
    let result = compiler::compileToFile(arg);
    match result {
        Ok(()) => println!("<> File successfully compiled..."),
        Err(_e) => println!("<> Unable to compile the file (invalid path)"),
    }
}

fn runFunction(arg: &str) {
    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: u16 = 0;
    let mut programCounter: u8 = 1;
    let mut flag: bool = false;

    let result = compiler::compiledFileToMem(&mut memory, arg);
    match result {
        Ok(()) => println!("<> Successfully running the file..."),
        Err(_e) => { println!("<> Unable to run the file (invalid path)"); return },
    }

    println!("---------------------------");

    let mut counter: i32 = 0;
    while memory[usize::from(programCounter)] != 0 && counter < 500 {
        instructions::executeInstruction(&mut programCounter, &mut accumulator, &mut flag, &mut memory);
        counter+=1;
    }

    println!("---------------------------");
    println!("<> Program hatled...");
}

fn main() {
    let mut command: String = String::new();

    while command != "q" {
        let mut consoleInput: String = String::new();
        std::io::stdin().read_line(&mut consoleInput).expect("error: unable to read user input");
        let tokens: Vec<&str> = consoleInput.split(" ").collect();
        let mut arg: String = String::new();

        if tokens.len() == 1 {
            command = tokens[0].replace("\r\n", "");
            arg = "".to_string();
        } else {
            command = tokens[0].to_string();
            for i in 1..tokens.len() {
                arg += &tokens[i].replace("\r\n", "");
                if i != tokens.len()-1 { arg += " "; }
            }
        }
  
        match command.as_str() {
           "help" => helpFunction(),
           "compile" => compileFunction(arg.as_str()),
           "run" => runFunction(arg.as_str()),
           _ => continue,
        }
    }
    println!("<> Closing the program...");
}