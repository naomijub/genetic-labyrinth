extern crate rand;

use rand::Rng;
use super::directions::{Directions,random_direction};

pub fn generate_genes() -> Vec<Directions>{
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(2, 12);
    (0..size).map(|_| random_direction()).collect::<Vec<Directions>>()
}

#[cfg(test)]
mod test {
  use super::generate_genes;
  use super::super::directions::Directions;

  fn gene_in_range(genes: Vec<Directions>) {
    assert!(genes.len() >= 2);
    assert!(genes.len() < 12);
  }

  #[test]
  fn genes_have_correct_sizing() {
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
    gene_in_range(generate_genes());
  }
}