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

    //Debug
    //println!("{}", &rom.len());

    let opcodes = match opcode_loader(&csv_path) {
        Ok(out) => out,
        Err(_) => {
            println!("Loading opcodes.csv failed.");
            Vec::new()
        },
    };

    //Debug
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

    let rom_assembly = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("invaders.txt")
        .expect("Unable to create disassembled file.");
    let mut rom_buffer = BufWriter::new(&rom_assembly);
    let mut i: usize = 0;
    loop {
        if i >= rom.len() {
            break
        }

        let byte = rom[i] as usize;
        let byte_size = opcodes[byte as usize].size;

        write!(&mut rom_buffer, "{}", opcodes[byte].comm)?;

        match byte_size {
            1 | 2 | 3 => {
                if byte_size > 1 {
                    write!(&mut rom_buffer, " $")?;
                }
                for j in Iterator::rev(2..=byte_size) {
                    write!(&mut rom_buffer, "{:02x}", rom[i+j as usize - 1])?;
                }
                //write!(&mut rom_buffer, "\n");
                writeln!(&mut rom_buffer, "")?;
                i += byte_size as usize;
            },
            len => {
                //into() converts String to the error type.
                return Err(format!("Invalid length of {}.", len).into())
            },

        }
        
    }
    Ok(())
}
