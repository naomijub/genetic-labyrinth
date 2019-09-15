extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use rand::seq::SliceRandom;
use super::genes::Gene;
use super::directions::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct Population {
  pub genes: Vec<Gene>,
  mutation_rate: f64,
  entrance: Point,
  lab: Vec<Vec<String>>
}

impl Population {
  pub fn new(mutation_rate: f64, entrance: Point, lab: Vec<Vec<String>>) -> Self {
    let genes = (0..20).map(|_| Gene::new().fitness_evaluator(lab.clone(), entrance.clone())).collect::<Vec<Gene>>();
    
    Population {
      genes: genes,
      mutation_rate: mutation_rate,
      entrance: entrance,
      lab: lab
    }
  }

  pub fn mutate_pop(self) -> Self {
    let new_genes = self.genes.clone()
      .into_par_iter()
      .map(|g| g.mutate_gene(self.mutation_rate.clone()))
      .map(|g| g.mutate_rna(self.mutation_rate.clone()))
      .map(|g| g.fitness_evaluator(self.lab.clone(), self.entrance.clone()))
      .collect::<Vec<Gene>>();

    Self { genes: new_genes, mutation_rate: self.mutation_rate, entrance: self.entrance, lab: self.lab }
  }

  pub fn crossing_over(self) -> Self {
    let pop_genes = self.clone().genes;
    let genes = (0..20)
      .map(move |_| crossover_genes(select_best(&pop_genes), select_best(&pop_genes)))
      .map(|g| g.fitness_evaluator(self.lab.clone(), self.entrance.clone()))
      .collect::<Vec<Gene>>();
    
    Self { genes: genes, mutation_rate: self.mutation_rate, entrance: self.entrance, lab: self.lab }
  }
}

fn crossover_genes(gene1: Gene, gene2: Gene) -> Gene {
  if gene1 != gene2 {
    let mut rna1 = gene1.clone().rna.split_at((gene1.clone().rna.len() as f64 / 2f64).ceil() as usize).0.to_vec();
    let rna2 = gene2.clone().rna.split_at((gene2.rna.len() as f64 / 2f64).floor() as usize).1.to_vec();

    rna1.extend(rna2);

    Gene {
      fitness: 0f64,
      rna: rna1
    }
  } else {
    vec![gene1, gene2].into_iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()).unwrap()
  }
}

fn select_best(genes: &Vec<Gene>) -> Gene {
  let selected_genes = genes
    .choose_multiple(&mut rand::thread_rng(), 3)
    .collect::<Vec<&Gene>>();

    get_best(&selected_genes)
}

fn get_best(selected_genes: &Vec<&Gene>) -> Gene {
  selected_genes
    .into_iter()
    .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    .unwrap()
    .clone()
    .to_owned()
}

pub fn best_gene(genes: &Vec<Gene>) -> Gene {
  genes
    .into_iter()
    .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    .unwrap()
    .clone()
    .to_owned()
}

#[cfg(test)]
mod test {
  use super::{Population, crossover_genes, select_best, get_best, best_gene};
  use super::super::directions::{Point,Directions};
  use super::super::genes::{Gene};
  use super::super::labreader::read_lab;

  // Main population functions
  #[test]
  fn new_is_created_with_20_genes() {
    let pop = Population::new(0.05f64, Point::from(1, 2), vec![vec!["1".to_string()], vec!["1".to_string()], vec!["1".to_string()]]);
    assert_eq!(pop.clone().genes.len(), 20);
    assert_eq!(pop.clone().entrance, Point::from(1, 2));
    assert_eq!(pop.clone().lab.len(), 3);
  }

  #[test]
  fn population_has_mutated_for_50percent() {
    let pop = Population::new(0.5f64, Point::from(0, 0), read_lab());
    let mutated_pop = pop.clone().mutate_pop();
    assert!(pop.genes != mutated_pop.genes);
  }

  #[test]
  fn population_has_crossed_over() {
    let pop = Population::new(0.5f64, Point::from(0, 0), read_lab());
    let crossedover_pop = pop.clone().crossing_over();
    assert!(pop.genes != crossedover_pop.genes);
  }

  #[test]
  fn best_gene_in_vec() {
    let gene1 = Gene{fitness: 10f64, rna: vec![Directions::N]};
    let gene2 = Gene{fitness: -100f64, rna: vec![Directions::W]};
    let gene3 = Gene{fitness: -10f64, rna: vec![Directions::S]};

    let genes = vec![gene1.clone(), gene2, gene3];

    assert_eq!(best_gene(&genes), gene1);
  }

  // Aux functions for crossing over
  #[test]
  fn select_a_result_from_3_vector() {
    let pop = Population::new(0.5f64, Point::from(0, 0), read_lab());
    let selected_gene = select_best(&pop.clone().genes);
    assert!(selected_gene.fitness > -100.0f64);
  }

   #[test]
  fn get_best_result_from_3_vector() {
    let gene1 = Gene{fitness: 10f64, rna: vec![Directions::N]};
    let gene2 = Gene{fitness: -100f64, rna: vec![Directions::W]};
    let gene3 = Gene{fitness: -10f64, rna: vec![Directions::S]};

    let genes = vec![&gene1, &gene2, &gene3];

    let selected_best = get_best(&genes);
    assert_eq!(selected_best.fitness, 10f64);
  }

  #[test]
  fn crossing_over_for_two_even_genes() {
    let gene1 = Gene{fitness: 10f64, rna: vec![Directions::N, Directions::W, Directions::S, Directions::N, Directions::E, Directions::S]};
    let gene2 = Gene{fitness: 10f64, rna: vec![Directions::W, Directions::NW, Directions::NE, Directions::N, Directions::SW, Directions::W, Directions::N, Directions::SE]};
    
    let actual_gene = crossover_genes(gene1, gene2);
    let expected_rna = vec![Directions::N, Directions::W, Directions::S, Directions::SW, Directions::W, Directions::N, Directions::SE];

    assert_eq!(actual_gene.rna, expected_rna);
  }

  #[test]
  fn crossing_over_for_two_odd_genes() {
    let gene1 = Gene{fitness: 10f64, rna: vec![Directions::N, Directions::W, Directions::S, Directions::N, Directions::E]};
    let gene2 = Gene{fitness: 10f64, rna: vec![Directions::NW, Directions::NE, Directions::N, Directions::SW, Directions::W, Directions::N, Directions::SE]};
    
    let actual_gene = crossover_genes(gene1, gene2);
    let expected_rna = vec![Directions::N, Directions::W, Directions::S, Directions::SW, Directions::W, Directions::N, Directions::SE];

    assert_eq!(actual_gene.rna, expected_rna);
  }

  #[test]
  fn crossing_over_for_one_even_one_odd_genes() {
    let gene1 = Gene{fitness: 10f64, rna: vec![Directions::N, Directions::W, Directions::S, Directions::N, Directions::E, Directions::S]};
    let gene2 = Gene{fitness: 10f64, rna: vec![Directions::NW, Directions::NE, Directions::N, Directions::SW, Directions::W, Directions::N, Directions::SE]};
    
    let actual_gene = crossover_genes(gene1, gene2);
    let expected_rna = vec![Directions::N, Directions::W, Directions::S, Directions::SW, Directions::W, Directions::N, Directions::SE];

    assert_eq!(actual_gene.rna, expected_rna);
  }
}