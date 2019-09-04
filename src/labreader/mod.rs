use std::fs::File;
use std::io::{prelude::*, BufReader};
use super::directions::Point;

pub fn read_lab() -> Vec<Vec<String>> {
  let file = File::open("resources/lab1.txt");
  match file {
    Ok(lab) => {
      let reader = BufReader::new(lab);
      let buffer_lines = reader.lines().skip(1);
      buffer_lines
        .map(|l| l.unwrap().trim()
                    .split("")
                    .map(|c| String::from(c))
                    .filter(|s| s != "")
                    .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
    },
    Err(_) => vec![]
  }
}

pub fn find_e(lines: Vec<Vec<String>>) -> Point {
  let py = lines.iter().position(|x| x.contains(&String::from("E")));
  let px = lines.get(py.clone().unwrap()).unwrap().iter().position(|x| x == "E");
  Point::from(px.unwrap() as i32, py.unwrap() as i32)
}

#[cfg(test)]
mod test {
  use super::{find_e, read_lab};
  use super::super::directions::Point;

  #[test]
  fn find_entrance() {
    let test_vec = vec![vec!["1".to_owned(), "3".to_owned(), "4".to_owned()], vec!["3".to_owned(), "E".to_owned(), "5".to_owned()], vec!["4".to_owned(), "5".to_owned(), "4".to_owned()]];
    let actual = find_e(test_vec);
    assert_eq!(Point::from(1,1), actual)
  }

  #[test]
  fn lab_format_is_10x10() {
    let actual = read_lab();
    let expected_len = 10;

    assert_eq!(expected_len, actual.len());
    assert_eq!(expected_len, actual.get(0).unwrap().len());
    assert_eq!(&String::from("E"), actual.get(0).unwrap().get(0).unwrap());
  }
}