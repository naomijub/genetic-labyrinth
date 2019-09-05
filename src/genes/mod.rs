extern crate rand;

use rand::Rng;
use super::directions::{Directions,random_direction, Point, movement};

const E: f32 = std::f32::consts::E;

type Rna = Vec<Directions>;

#[derive(Clone)]
struct Gene {
  rna: Rna,
  fitness: f32,
}

impl Gene {
  fn new() -> Gene {
    Gene {
      rna: generate_genes_rna(),
      fitness: 0f32,
    }
  }

  fn fitness_evaluator(self, lab: Vec<Vec<String>>, entrance: Point) -> Self {
    let path = movement(lab, entrance, self.rna.clone());
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

 fn fitness_calculator(value: i32) -> f32 {
   let x = value as f32;
    3f32 * ((E.powf(x - 0.5f32) - 1f32)/(E.powf(x - 0.5f32) + 1f32) 
      * (E.powf(x + 0.5f32) - 1f32)/(E.powf(x + 0.5f32) + 1f32) * (-20f32)).tanh() - 2f32
  }
  
  fn fitness(values: Vec<i32>, has_found_exit: bool) -> f32 {
    let exit_bonus = if has_found_exit { 10f32 } else { -1f32 };
    values.into_iter()
      .map(|v| fitness_calculator(v))
      .fold(0f32,|acc, v| acc + v) + exit_bonus
  }

pub fn generate_genes_rna() -> Rna{
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(2, 12);
    (0..size).map(|_| random_direction()).collect::<Rna>()
}

#[cfg(test)]
mod test {
  use super::{generate_genes_rna, Rna, Gene, fitness, fitness_calculator};

  fn gene_in_range(genes: Rna) {
    assert!(genes.len() >= 2);
    assert!(genes.len() < 12);
  }

  #[test]
  fn gene_is_created_with_fitness_0() {
    let gene = Gene::new();
    assert_eq!(0f32, gene.fitness);
    assert!(gene.rna.len() >= 2);
  }

  #[test]
  fn test_fitness_value_spectrum() {
    assert!(-3f32 > fitness_calculator(-1));
    assert!(-3f32 > fitness_calculator(1));
    assert!(0.5f32 < fitness_calculator(0));
  }

  #[test]
  fn test_fitness_from_genes_without_exit() {
    assert!(-15f32 > fitness(vec![-1, 1, 0, 0, 1, 1, 0], false));
  }

  #[test]
  fn test_fitness_from_genes_with_exit() {
    assert!(-10f32 < fitness(vec![-1, 1, 0, 0, 1, 1, 0], true));
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
}