use std::env;
use std::fs;
use std::process;

use bf::compiler::Compiler;
use bf::machine::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);

            let mut compiler = Compiler::new(contents.as_str());
            let program = compiler.compile();
            program.iter().for_each(|e| println!("{:?}",e));
            let mut m = Machine::new(program);
            m.execute();
            println!("Execution done");
        }
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error);
            process::exit(1);
        }
    }
}
