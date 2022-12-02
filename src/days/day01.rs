use std::str::FromStr;

#[inline(always)]
pub fn day1() -> u32 {
  let contents = include_str!("../../inputs/day01.txt");
  let mut result = contents
    .split("\n\n")
    .into_iter()
    .map(|x| x.split('\n').into_iter())
    .map(|x| x.fold(0, |acc, x| acc + u32::from_str(x).unwrap()))
    .collect::<Vec<_>>();

  result.sort_by(|a, b| b.cmp(a));

  (&result[0..3]).into_iter().sum()
}
