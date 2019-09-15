mod labreader;
mod directions;
mod genes;
mod population;

use std::io::{self};
use labreader::{read_lab, find_e};
use population::{Population, best_gene};
use genes::{Gene};

fn main() -> io::Result<()> {
    let lines = read_lab();
    let entrance = find_e(lines.clone());
    let mut pop = Population::new(0.5f64, entrance, read_lab());
    let mut best_value = -1000f64;
    let mut gene = Gene::new();

    while best_value < 15f64 {
        let current_pop = pop.clone().mutate_pop().crossing_over();
        let pop_best_gene = best_gene(&current_pop.genes);
        best_value = pop_best_gene.clone().fitness;
        gene = pop_best_gene;
        pop = current_pop;
    }

    println!("{:?}", gene);
    Ok(())
}
