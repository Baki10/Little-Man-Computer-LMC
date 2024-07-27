

pub fn executeInstruction(pc: &mut u8, acc: &mut u16, flag: &mut bool, mem: &mut Vec<u16>) {
    let index: u8 = *pc;
    let instruction: u8 = (mem[usize::from(index)]/100) as u8;
    let address: u8 = (mem[usize::from(index)]%100) as u8;
    let mut value: &mut u16 = &mut mem[usize::from(address)];

    match instruction {
        1=>add(acc, flag, value),
        2=>sub(acc, flag, value),
        3=>store(acc, &mut value),
        5=>load(acc, flag,value),
        6=>branch(pc, &address),
        7=>branchZero(pc, &address, acc, flag),
        8=>branchPositive(pc, &address, flag),
        9=>io(&address, acc),
        _=>println!("error : invalid instruction"),
    }
    
    if instruction < 6 || instruction > 8 {
        *pc += 1;
    }
}

fn io(adr: &u8, acc: &mut u16) {
    match *adr {
        1=>inp(acc),
        2=>out(acc),
        _=>println!("error : invalid address"),
    }
}
fn add(acc: &mut u16, flag: &mut bool, val: &u16) {
    *acc += *val;
    if *acc > 999 {
        *flag = !*flag;
        *acc %= 1000;
    }
}
fn sub(acc: &mut u16, flag: &mut bool, val: &u16) {
    
    if *acc >= *val { 
        *acc -= *val;
    } else {
        *flag = !*flag;
        *acc = 1000 - (*val - *acc);
    } 
}
fn store(acc: &u16, val: &mut u16) {
    *val = *acc;
}
fn load(acc: &mut u16, flag: &mut bool, val: &u16) {
    *acc = *val;
    *flag = false;
}
fn branch(pc: &mut u8, adr: &u8) {
    *pc = *adr;
}
fn branchZero(pc: &mut u8, adr: &u8, acc: &u16, flag: &bool) {
    if *acc == 0 && *flag == false {
        *pc = *adr;
    } else {
        *pc += 1;
    }
}
fn branchPositive(pc: &mut u8, adr: &u8, flag: &bool) {
    if *flag == false {
        *pc = *adr;
    } else {
        *pc += 1;
    }
}
fn inp(acc: &mut u16) {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).expect("error: unable to read user input");
    *acc = str.trim().parse().expect("error: input not an integer");
}
fn out(acc: &u16) {
    println!("{}", acc);
}