mod labreader;
mod directions;
mod genes;

use std::io::{self};
use labreader::{read_lab, find_e};

fn main() -> io::Result<()> {
    let lines = read_lab();
    let entrance = find_e(lines);
    

    Ok(())
}
