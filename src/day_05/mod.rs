use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Crane {
  stacks: Vec<Vec<char>>,
  is_newer_model: bool,
}

impl<'a> Crane {
  fn new(stacks: Vec<&'a str>, is_newer_model: bool) -> Self {
    return Self {
      stacks: stacks
        .iter()
        .map(|stack| stack.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>(),
      is_newer_model,
    };
  }

  fn move_crate(&mut self, amount: usize, from: usize, to: usize) {
    let stack_from = self.stacks.get_mut(from).unwrap();

    let from_length = stack_from.len();

    let mut tmp = stack_from
      .splice((from_length - amount)..from_length, [])
      .collect::<Vec<_>>();

    /*
     * This is the only difference between part 1 and part 2
     * part 1 moves them one by one, which reverses the order,
     * part 2 can move in bulk, which preserves order
     */
    if !self.is_newer_model {
      tmp.reverse();
    }

    self.stacks.get_mut(to).unwrap().append(&mut tmp);
  }

  fn get_crates_on_top(&self) -> String {
    return self
      .stacks
      .iter()
      .map(|stack| stack.last().unwrap().to_string())
      .collect::<Vec<_>>()
      .join("");
  }
}

pub fn run_day_05_part_1() {
  let regex = Regex::new(r"move (\d+?) from (\d+?) to (\d+?)").unwrap();

  match fs::read_to_string("./src/day_05/input") {
    Ok(input) => {
      #[rustfmt::skip]
      let mut stacks = Crane::new([
        /* [1] */ "NBDTVGZJ",
        /* [2] */ "SRMDWPF",
        /* [3] */ "VCRSZ",
        /* [4] */ "RTJZPHG",
        /* [5] */ "TCJNDZQF",
        /* [6] */ "NVPWGSFM",
        /* [7] */ "GCVBPQ",
        /* [8] */ "ZBPN",
        /* [9] */ "WPJ",
      ].to_vec(), false);

      for line in input.lines() {
        let (_, [amount, from, to]) = regex.captures(line).unwrap().extract();
        let amount = amount.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();
        stacks.move_crate(amount, from - 1, to - 1);
      }

      println!("result: {}", stacks.get_crates_on_top());
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_05_part_2() {
  let regex = Regex::new(r"move (\d+?) from (\d+?) to (\d+?)").unwrap();

  match fs::read_to_string("./src/day_05/input") {
    Ok(input) => {
      #[rustfmt::skip]
      let mut stacks = Crane::new([
        /* [1] */ "NBDTVGZJ",
        /* [2] */ "SRMDWPF",
        /* [3] */ "VCRSZ",
        /* [4] */ "RTJZPHG",
        /* [5] */ "TCJNDZQF",
        /* [6] */ "NVPWGSFM",
        /* [7] */ "GCVBPQ",
        /* [8] */ "ZBPN",
        /* [9] */ "WPJ",
      ].to_vec(), true);

      for line in input.lines() {
        let (_, [amount, from, to]) = regex.captures(line).unwrap().extract();
        let amount = amount.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();
        stacks.move_crate(amount, from - 1, to - 1);
      }

      println!("result: {}", stacks.get_crates_on_top());
    }
    Err(error) => panic!("error: {}", error),
  };
}
