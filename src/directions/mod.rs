extern crate rand;

use std::ops::Add;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq,Eq, Debug, Clone)]
pub struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {0: self.0 + other.0, 1: self.1 + other.1}
    }
}

impl Point {
  pub fn from(x: i32, y: i32) -> Point {
    Point{0: x, 1: y}
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Directions {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SW,
    SE
}

impl Distribution<Directions> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Directions {
        match rng.gen_range(0, 8) {
          0  => Directions::N,
          1  => Directions::S,
          2  => Directions::W,
          3  => Directions::E,
          4  => Directions::NE,
          5  => Directions::NW,
          6  => Directions::SE,
          _  => Directions::SW,
        }
    }
}

pub fn random_direction() -> Directions {
  let dir: Directions = rand::random();
  dir
}

pub fn direction_to_position(direction: Directions) -> Point {
  match direction {
    Directions::N => Point(0, -1),
    Directions::S => Point(0, 1),
    Directions::W => Point(-1, 0),
    Directions::E => Point(1, 0),
    Directions::NE => Point(1, -1),
    Directions::NW => Point(-1, -1),
    Directions::SE => Point(1, 1),
    Directions::SW => Point(-1, 1)
  }
}

pub fn movement(lab: Vec<Vec<String>>, entrance: Point, directions: Vec<Directions>) -> Vec<String> {
  let init = entrance.clone();
  let empty_vec = Vec::new();
  let default = "-1".to_string();
  let movements = directions.into_iter()
    .map(|d| direction_to_position(d))
    .fold((init , vec![String::from("E")]), |mut acc, d| {
      let next_pos = acc.0 + d;
      let next_value = lab.get(next_pos.1 as usize).unwrap_or(&empty_vec).get(next_pos.0 as usize).unwrap_or(&default);
      acc.1.push(next_value.to_string());
      (next_pos, acc.1)
    });
  movements.1
}

#[cfg(test)]
mod test {
  use super::{Directions,direction_to_position, movement, Point, random_direction};
  use super::super::labreader::read_lab;

  fn is_dir(dir: Directions) -> bool {
    match dir {
      Directions::N => true,
      Directions::S => true,
      Directions::W => true,
      Directions::E => true,
      Directions::NE => true,
      Directions::NW => true,
      Directions::SE => true,
      Directions::SW => true,
      _ => false,
    }
  }

  #[test]
  fn maps_direction_to_position() {
    assert_eq!(direction_to_position(Directions::N), Point(0, -1));
    assert_eq!(direction_to_position(Directions::S), Point(0, 1));
    assert_eq!(direction_to_position(Directions::W), Point(-1, 0));
    assert_eq!(direction_to_position(Directions::E), Point(1, 0));
    assert_eq!(direction_to_position(Directions::NE), Point(1, -1));
    assert_eq!(direction_to_position(Directions::NW), Point(-1, -1));
    assert_eq!(direction_to_position(Directions::SE), Point(1, 1));
    assert_eq!(direction_to_position(Directions::SW), Point(-1, 1));
  }

  #[test]
  fn moves_one_step_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::S];
    let expected = vec!["E", "1"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }

  #[test]
  fn random_direction_is_available() {
    assert!(is_dir(random_direction()));
  }

  #[test]
  fn moves_two_step_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::S, Directions::E];
    let expected = vec!["E", "1", "0"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }

  #[test]
  fn moves_into_void_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::S, Directions::W];
    let expected = vec!["E", "1", "-1"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }

  #[test]
  fn moves_twice_into_void_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::N, Directions::W];
    let expected = vec!["E", "-1", "-1"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }

  #[test]
  fn find_s_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::S, Directions::S, Directions::S, Directions::S, Directions::S, Directions::S, Directions::S];
    let expected = vec!["E", "1", "0", "0", "0", "1", "1", "S"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }

  #[test]
  fn find_0s_on_lab1() {
    let lab1 = read_lab();
    let directions = vec![Directions::E, Directions::S, Directions::S, Directions::E, Directions::E, Directions::S];
    let expected = vec!["E", "0", "0", "0", "0", "0", "0"];

    let actual = movement(lab1, Point(0,0), directions);

    assert_eq!(expected, actual);
  }
}


