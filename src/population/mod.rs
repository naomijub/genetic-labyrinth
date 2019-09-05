use super::genes::Gene;
use super::directions::Point;

#[derive(Clone)]
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
}

#[cfg(test)]
mod test {
  use super::{Population};
  use super::super::directions::Point;

  #[test]
  fn new_is_created_with_20_genes() {
    let pop = Population::new(0.05f32, Point::from(1, 2), vec![vec!["1".to_string()], vec!["1".to_string()], vec!["1".to_string()]]);
    assert_eq!(pop.clone().genes.len(), 20);
    assert_eq!(pop.clone().entrance, Point::from(1, 2));
    assert_eq!(pop.clone().lab.len(), 3);
  }
}