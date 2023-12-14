use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
struct Args {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let argumente = Args::parse();

    if let (Some(input), Some(output)) = (argumente.input, argumente.output) {
        let mut buffer = Vec::new();
        File::open(Path::new(&input))
            .unwrap()
            .read_to_end(&mut buffer)
            .unwrap();
        let codificat = base64::codifica_base64(&buffer);
        File::create(Path::new(&output))
            .unwrap()
            .write_all(codificat.as_bytes())
            .unwrap();
    } else {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let codificat = base64::codifica_base64(buffer.as_bytes());
        println!("{}", codificat);
    }
}
