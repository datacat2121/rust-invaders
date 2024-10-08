use std::fs;
//use std::io;
//use std::io::BufReader;
use std::io::BufWriter;
//use std::io::Read;
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Opcode {
    /*#[serde(rename = "Opcode")]
    opcode: u8,*/
    #[serde(rename = "Instruction")]
    comm: String,
    size: u8,
}

fn main() {

    //This is the first 128 bytes of the rom.
    //let rom_path = String::from("invaders128");
    let rom_path = String::from("invaders.h");
    let csv_path = String::from("opcodes.csv");
    let rom: Vec<u8> = loop {
        //Uncomment below to ask for rom file.
        /*let mut rom_path = String::new();

        io::stdin()
            .read_line(&mut rom_path)
            .expect("Failed to read input");

        let rom_path: String = match rom_path.trim().parse() {
            Ok(path) => path,
            Err(_) => {
                println!("Not a valid file.");
                continue;
            },
        };*/
        let rom = match fs::read(&rom_path) {
            Ok(out) => out,
            Err(_) => { 
                println!("Unable to load ROM.");
                continue
            }
        };
        break rom
    };

    //println!("{}", &rom.len());

    let opcodes = match opcode_loader(&csv_path) {
        Ok(out) => out,
        Err(_) => {
            println!("Loading opcodes.csv failed.");
            Vec::new()
        },
    };

    //println!("{:?}", &opcodes);

    match disassembler(&rom, &opcodes){
        Ok(_) => {
            println!("ROM disassembled successfully!")
        },
        Err(_) => {
            println!("ROM disassembly failed.")
        },
    };
}

fn opcode_loader(file: &String) -> Result<Vec<Opcode>, Box<dyn std::error::Error>> {
    let mut opcode_vec: Vec<Opcode> = Vec::new();
    let file = fs::File::open(file)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let entry: Opcode = result?;
        opcode_vec.push(entry);
    }
    Ok(opcode_vec)

}

fn disassembler(rom: &Vec<u8>, opcodes: &Vec<Opcode>) -> Result<(), Box<dyn std::error::Error>> {
    //let rom_assembly = fs::File::create("invaders.txt")?;
    let rom_assembly = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("invaders.txt")
        .expect("Unable to create disassembled file.");
    let mut rom_buffer = BufWriter::new(&rom_assembly);
    let mut i: usize = 0;
    loop {
        /*if i >= rom.len() as i32 + 1 {
            break
        }*/
        write!(&mut rom_buffer, "{}", opcodes[rom[i] as usize].comm).expect("Unable to write into buffer.");
        match opcodes[rom[i] as usize].size {
            1 => {
                write!(&mut rom_buffer, "\n").expect("Unable to write into buffer.");
                i += 1;
                if i >= rom.len() {
                    break
                }
                continue
            },
            2 => {
                write!(&mut rom_buffer, " {}\n", rom[i+1]).expect("Unable to write into buffer.");
                i += 2;
                if i >= rom.len() {
                    break
                }
                continue
            },
            3 => {
                write!(&mut rom_buffer, " {} {}\n", rom[i+1], rom[i+2]).expect("Unable to write into buffer.");
                i += 3;
                if i >= rom.len() {
                    break
                }
                continue
            },
            _ => {},

        }
        
    }
    Ok(())
}
/*
//What's happening here?
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("opcodes.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Opcode = result?;
        println!("{:?}", record);
    }
    Ok(())
}*/
