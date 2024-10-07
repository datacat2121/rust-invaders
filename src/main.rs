use std::fs;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Opcode {
    #[serde(rename = "Opcode")]
    opcode: u8,
    #[serde(rename = "Instruction")]
    comm: String,
    size: u8,
}

fn main() {

    //This is the first 128 bytes of the rom.
    let rom_path = String::from("invaders128");
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

    println!("{}", rom.len());

    //Error handling?
    if let Err(err) = example() {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
}

fn opcode_loader(file: String) /*-> Vec<Opcode>*/ {
    let csv: fs::File = loop {
        let csv = match fs::File::open(&file) {
            Ok(out) => out,
            Err(_) => {
                println!("Unable to load opcodes.");
                continue
            }
        };
        break csv
    };
}

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
}
