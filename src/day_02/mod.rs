use std::fs;

pub fn run_day_02_part_1() {
  #[rustfmt::skip]
  let points = vec![
    /* outcome if 1 vs 1 */ 3,
    /* outcome if 1 vs 2 */ 6,
    /* outcome if 1 vs 3 */ 0,

    /* outcome if 2 vs 1 */ 0,
    /* outcome if 2 vs 2 */ 3,
    /* outcome if 2 vs 3 */ 6,

    /* outcome if 3 vs 1 */ 6,
    /* outcome if 3 vs 2 */ 0,
    /* outcome if 3 vs 3 */ 3
  ];

  match fs::read_to_string("./src/day_02/input") {
    Ok(input) => {
      let mut score = 0;
      for line in input.lines() {
        let mut line = line.split_whitespace().map(|play| play.as_bytes()[0]);

        let opponent = (line.next().unwrap() - b'A') as usize;
        let attacker = (line.next().unwrap() - b'Z') as usize;

        let point = points[opponent * 3 + attacker];

        score += (attacker + 1) + point
      }

      println!("score {}", score);
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_02_part_2() {
  #[rustfmt::skip]
  let attacker = [
    // LOSS
    /* if opponent uses 1 then */ 3, 
    /* if opponent uses 2 then */ 1, 
    /* if opponent uses 3 then */ 2, 

    // DRAW
    /* if opponent uses 1 then */ 1, 
    /* if opponent uses 2 then */ 2, 
    /* if opponent uses 3 then */ 3, 

    // WIN
    /* if opponent uses 1 then */ 2, 
    /* if opponent uses 2 then */ 3, 
    /* if opponent uses 3 then */ 1
    ];

  match fs::read_to_string("./src/day_02/input") {
    Ok(input) => {
      let mut score = 0;
      for line in input.lines() {
        let mut line = line.split_whitespace().map(|play| play.as_bytes()[0]);

        let opponent = (line.next().unwrap() - b'A') as usize;
        let outcome = (line.next().unwrap() - b'X') as usize;

        let point = attacker[outcome * 3 + opponent] + outcome * 3;

        score += point;
      }

      println!("score {}", score);
    }
    Err(error) => panic!("error: {}", error),
  };
}
