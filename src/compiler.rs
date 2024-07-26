use std::{fs, io::Write};


pub fn compileToFile(filePath: &str) -> std::io::Result<()> {
    let fileContent = fs::read_to_string(filePath).expect("error: unable to read the file");
    let lines = fileContent.split("\n");
    let mut outputCode = String::new();

    for line in lines {
        let decodedLine = decodeLine(line);
        outputCode.push_str(&decodedLine);
        outputCode.push_str("\n");
    }

    let mut newPath = filePath.to_string();
    newPath.replace_range(newPath.len()-3..newPath.len(), "lmc");

    let mut outputFile = fs::File::create(newPath)?;
    outputFile.write_all(outputCode.as_bytes()).expect("error: unable to write to the file");
    Ok(())
}

fn decodeLine(line: &str) -> String {

    let mut decodedLine: String = String::new();
    let splitLine: Vec<&str> = line.split(" ").collect();
    let cleandLine = splitLine[0].replace("\r","");
    let command: &str = cleandLine.as_str();
    let arg = splitLine[splitLine.len()-1].replace("\r","");

    match command {
        "ADD" => decodedLine.push_str(&format!("1{}", arg)),
        "SUB" => decodedLine.push_str(&format!("2{}", arg)),
        "STA" => decodedLine.push_str(&format!("3{}", arg)),
        "LDA" => decodedLine.push_str(&format!("5{}", arg)),
        "BRA" => decodedLine.push_str(&format!("6{}", arg)),
        "BRZ" => decodedLine.push_str(&format!("7{}", arg)),
        "BRP" => decodedLine.push_str(&format!("8{}", arg)),
        "INP" => decodedLine.push_str("901"),
        "OUT" => decodedLine.push_str("902"),
        "HLT" => decodedLine.push_str("000"),
        _ => return command.to_string(),
    }

    return decodedLine;
}

pub fn compiledFileToMem(mem: &mut Vec<u16>, filePath: &str) {
    let fileContent: String = fs::read_to_string(filePath).expect("error: unable to read the file");
    let lines = fileContent.split("\n");

    let mut nextMem = 1;
    for line in lines {
        if(line != "") {
            mem[nextMem] = line.parse::<u16>().unwrap();
        }
        nextMem += 1;
        if(nextMem > 99) { break; }
    }
}