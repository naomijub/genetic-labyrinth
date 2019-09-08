use super::genes::Gene;
use super::directions::Point;

#[derive(Clone, PartialEq)]
struct Population {
  genes: Vec<Gene>,
  mutation_rate: f32,
  entrance: Point,
  lab: Vec<Vec<String>>
}

impl Population {
  pub fn new(mutation_rate: f32, entrance: Point, lab: Vec<Vec<String>>) -> Self {
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
      .into_iter()
      .map(|g| g.mutate_gene(self.mutation_rate.clone()))
      .map(|g| g.mutate_rna(self.mutation_rate.clone()))
      .map(|g| g.fitness_evaluator(self.lab.clone(), self.entrance.clone()))
      .collect::<Vec<Gene>>();

    Self { genes: new_genes, mutation_rate: self.mutation_rate, entrance: self.entrance, lab: self.lab }
  }
}

#[cfg(test)]
mod test {
  use super::{Population};
  use super::super::directions::Point;
  use super::super::labreader::read_lab;

  #[test]
  fn new_is_created_with_20_genes() {
    let pop = Population::new(0.05f32, Point::from(1, 2), vec![vec!["1".to_string()], vec!["1".to_string()], vec!["1".to_string()]]);
    assert_eq!(pop.clone().genes.len(), 20);
    assert_eq!(pop.clone().entrance, Point::from(1, 2));
    assert_eq!(pop.clone().lab.len(), 3);
  }

  #[test]
  fn population_has_mutated_for_50percent() {
    let pop = Population::new(0.5f32, Point::from(0, 0), read_lab());
    let mutated_pop = pop.clone().mutate_pop();
    assert!(pop.genes != mutated_pop.genes);
  }
}