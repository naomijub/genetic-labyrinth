mod labreader;
mod directions;
mod genes;
mod population;

use std::io::{self};
use labreader::{read_lab, find_e};
use genes::{Gene};

fn main() -> io::Result<()> {
    let lines = read_lab();
    let entrance = find_e(lines.clone());
    let gene = Gene::new().fitness_evaluator(lines, entrance);

    println!("{:?}", gene);

    Ok(())
}
