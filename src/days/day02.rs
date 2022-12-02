use hashbrown::HashMap;

lazy_static::lazy_static! {
  // Used in part 1
  static ref OUTCOME_LUT: HashMap<&'static str, u32> = HashMap::from([
    ("A X", 1+3), 
    ("A Y", 2+6), 
    ("A Z", 3+0),
    ("B X", 1+0), 
    ("B Y", 2+3), 
    ("B Z", 3+6),
    ("C X", 1+6),
    ("C Y", 2+0),
    ("C Z", 3+3),
  ]);

  // Rearranged OUTCOME_LUT to be used for part 2
  static ref DECISION_LUT: HashMap<&'static str, u32> = HashMap::from([
    ("A Z", OUTCOME_LUT["A Y"]), 
    ("A Y", OUTCOME_LUT["A X"]), 
    ("A X", OUTCOME_LUT["A Z"]),
    ("B Z", OUTCOME_LUT["B Z"]),
    ("B Y", OUTCOME_LUT["B Y"]),
    ("B X", OUTCOME_LUT["B X"]),
    ("C Z", OUTCOME_LUT["C X"]),
    ("C Y", OUTCOME_LUT["C Z"]),
    ("C X", OUTCOME_LUT["C Y"]),
  ]);
}

#[inline(always)]
pub fn day2() -> u32 {
  let contents = include_str!("../../inputs/day02.txt");
  let results = contents
    .split('\n')
    .fold(0, |acc, x| {
      acc + DECISION_LUT[x]
    });
    
  results
}