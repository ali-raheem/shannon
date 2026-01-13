use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use textplots::{Chart, Plot, Shape};

use shannon::entropy;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
    #[clap(long, short, default_value_t = 1024)]
    block_size: usize,
    #[clap(long, default_value_t = 180)]
    width: u32,
    #[clap(long, default_value_t = 100)]
    height: u32,
    #[clap(long, short)]
    y_max: Option<f32>,
}

fn main() {
    let args = Args::parse();
    if args.width < 32 || args.height < 32 {
        println!("Width and Height must be atleast 32.");
        process::exit(1);
    }
    let f = match File::open(&args.input_file) {
	Ok(f) => f,
	Err(e) => {
	    println!("Couldn't open file {} got error {e}.", args.input_file);
	    process::exit(1);
	},
    };
    let mut reader = BufReader::new(f);

    let mut read_buffer = vec![0u8; args.block_size];
    let mut s = Vec::new();
    loop {
        let len = match reader.read(&mut read_buffer) {
	    Ok(l) => l,
	    Err(e) => {
		println!("Unexpecedtly could not read from file {}, got error {e}", args.input_file);
		process::exit(1);
	    }
	};
        if len == 0 {
            break;
        }
        s.push(entropy::<f32>(&read_buffer[..len]));
    }
    let x_max = s.len() as f32;
    let y_max = args
        .y_max
        .unwrap_or_else(|| s.iter().fold(0.0_f32, |a, &y| a.max(y)));

    let s: Vec<(f32, f32)> = s
        .into_iter()
        .enumerate()
        .map(|(x, y)| (x as f32, y))
        .collect();

    Chart::new_with_y_range(args.width, args.height, 0.0, x_max, 0.0, y_max)
        .lineplot(&Shape::Bars(&s))
        .display();
}
