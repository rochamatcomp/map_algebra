/// Map algebra from raster files.

extern crate raster_mapping;

use std::collections::HashMap;
use raster_mapping::{Mapping, Raster};

fn arguments() -> HashMap<String, f32>{

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() % 2 != 0{
        println!("Usage:");
        println!("Sets the pairs: input files and their respective weights. Example: file1 weight1 file2 weight2 file3 weight3 ...");
        std::process::exit(0);
    }
    
    // Gets the pair (key, value) in the arguments.
    let pair = args.chunks(2);
    let mut result: HashMap<String, f32> = HashMap::new();

    for arg in pair {
        let key: String  = arg[0].parse().unwrap();
        let value: f32 = arg[1].parse().unwrap();
        result.insert(key, value);
    }
    result
}

fn main() {
    let maps: HashMap<String, f32> = arguments();    
    let result: Raster<f32> = Raster::algebra(maps);
    println!("{:?}", result);
}
