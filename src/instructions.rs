
pub fn add(acc: &mut u16, flag: &mut bool, val: &u16) {
    *acc += *val;
    if(*acc > 999) { *flag = !*flag; *acc %= 1000;}
}

pub fn sub(acc: &mut u16, flag: &mut bool, val: &u16) {
    
    if(*acc >= *val) { 
        *acc -= *val;
    }
    else {
        *flag = !*flag;
        *acc = 1000 - (*val - *acc);
    } 
}

pub fn store(acc: &u16, val: &mut u16) {
    *val = *acc;
}

pub fn load(acc: &mut u16, flag: &mut bool, val: &u16) {
    *acc = *val;
    *flag = false;
}

pub fn branch(pc: &mut u8, adr: &u8) {
    *pc = *adr;
}

pub fn branchZero(pc: &mut u8, adr: &u8, acc: &u16, flag: &bool) {
    if(*acc == 0 && *flag == false)
    {
        *pc = *adr;
    }
}

pub fn branchPositive(pc: &mut u8, adr: &u8, flag: &bool) {
    if(*flag == false)
    {
        *pc = *adr;
    }
}

pub fn inp(acc: &mut u16) {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).expect("error: unable to read user input");
    *acc = str.trim().parse().expect("error: input not an integer");
}

pub fn out(acc: &u16) {
    println!("{}", acc);
}