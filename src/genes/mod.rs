extern crate rand;

use rand::Rng;
use super::directions::{Directions,random_direction, Point, movement};

const E: f64 = std::f64::consts::E;

type Rna = Vec<Directions>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Gene {
  pub rna: Rna,
  pub fitness: f64,
}

impl Gene {
  pub fn new() -> Gene {
    Gene {
      rna: generate_genes_rna(),
      fitness: 0f64,
    }
  }

  pub fn mutate_gene(mut self, mutation_rate: f64) -> Self {
    let mut rng = rand::thread_rng();
    let rna_len = &self.rna.len();
    let rnd_index = rng.gen_range(0, rna_len.to_owned());
    let generated_percentile = rng.gen_range(0f64, 1f64);
    if mutation_rate > generated_percentile {
      self.rna.remove(rnd_index);
      self.rna.insert(rnd_index, random_direction());
      self
    } else {
      self
    }
  }

  pub fn mutate_rna(mut self, mutation_rate: f64) -> Self {
    let mut rng = rand::thread_rng();
    let generated_percentile = rng.gen_range(0f64, 1f64);
    if mutation_rate > generated_percentile {
      self.rna.pop();
      self
    } else if 1f64 - mutation_rate < generated_percentile {
      self.rna.push(random_direction());
      self
    } else {
      self
    }
  }

  pub fn fitness_evaluator(self, lab: &Vec<Vec<String>>, entrance: &Point) -> Self {
    let path = movement(lab, entrance, &self.rna);
    let values = path.iter()
      .map(|rna| match &rna[..] {
        "E" => 0,
        "-1" => -1,
        "1" => 1,
        "0" => 0,
        "S" => 0,
        _ => -1,
      })
      .collect::<Vec<i32>>();
    let has_found_exit = path.contains(&"S".to_string());

    Self {
      rna: self.rna,
      fitness: fitness(values, has_found_exit)
    }
  }
}

 fn fitness_calculator(value: &i32) -> f64 {
   let x = value.to_owned() as f64;
    4f64 * ((E.powf(x - 0.5f64) - 1f64)/(E.powf(x - 0.5f64) + 1f64) 
      * (E.powf(x + 0.5f64) - 1f64)/(E.powf(x + 0.5f64) + 1f64) * (-20f64)).tanh() - 3f64
  }
  
  fn fitness(values: Vec<i32>, has_found_exit: bool) -> f64 {
    let exit_bonus = if has_found_exit { 10f64 } else { -5f64 };
    values.clone().iter()
      .map(|v| fitness_calculator(v))
      .fold(0f64,|acc, v| acc + v) + exit_bonus - 10f64.powi(values.len() as i32 - 10i32)
  }

fn generate_genes_rna() -> Rna {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(2, 12);
    (0..size).map(|_| random_direction()).collect::<Rna>()
}

#[cfg(test)]
mod test {
  use super::{generate_genes_rna, Rna, Gene, fitness, fitness_calculator};
  use super::super::directions::{Directions,Point};

  fn gene_in_range(genes: Rna) {
    assert!(genes.len() >= 2);
    assert!(genes.len() < 12);
  }

  #[test]
  fn gene_is_created_with_fitness_0() {
    let gene = Gene::new();
    assert_eq!(0f64, gene.fitness);
    assert!(gene.rna.len() >= 2);
  }

  #[test]
  fn evaluate_fitness_from_gene() {
    let gene = Gene {rna: vec![Directions::E, Directions::S, Directions::S] ,fitness: 0f64};
    let actual = gene.fitness_evaluator(&vec![vec!["E".to_string(), "0".to_string(), "1".to_string()], vec!["1".to_string(), "0".to_string(), "0".to_string()], vec!["1".to_string(), "S".to_string(), "1".to_string()]], &Point::from(0i32, 0i32));
    assert!(actual.fitness >= 8f64);
  }

  #[test]
  fn test_fitness_value_spectrum() {
    assert!(-6f64 > fitness_calculator(&-1));
    assert!(-6f64 > fitness_calculator(&1));
    assert!(0f64 < fitness_calculator(&0));
  }

  #[test]
  fn test_fitness_from_genes_without_exit() {
    assert!(-15f64 > fitness(vec![-1, 1, 0, 0, 1, 1, 0], false));
  }

  #[test]
  fn test_fitness_from_genes_with_exit() {
    assert!(-15f64 >= fitness(vec![-1, 1, 0, 0, 1, 1, 0], true));
  }

  #[test]
  fn genes_have_correct_sizing() {
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
    gene_in_range(generate_genes_rna());
  }

  #[test]
  fn mutation_when_110percent() {
    let gene = Gene::new();
    assert!(gene.clone().mutate_gene(1.1f64).rna != gene.rna);  
  }

  #[test]
  fn no_mutation_when_0percent() {
    let gene = Gene::new();
    assert_eq!(gene.clone().mutate_gene(-0.1f64).rna, gene.rna);  
  }

  #[test]
  fn mutation_rna_size_when_100percent() {
    let gene = Gene::new();
    assert!(gene.clone().mutate_rna(1f64).rna.len() != gene.rna.len());  
  }

  #[test]
  fn no_mutation_rna_size_when_0percent() {
    let gene = Gene::new();
    assert_eq!(gene.clone().mutate_rna(-0.1f64).rna.len(), gene.rna.len());  
  }
}