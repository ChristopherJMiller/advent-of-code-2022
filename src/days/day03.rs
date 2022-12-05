use hashbrown::{HashMap};


#[inline(always)]
pub fn char_to_priority(c: char) -> u32 {
  let subtractor = if (c as usize) > 90 {
    96
  } else {
    38
  };

  (c as u32) - subtractor
}

#[inline(always)]
pub fn day3() -> u32 {
  let contents = include_str!("../../inputs/day03.txt");

  contents
    .split('\n')
    .map(|x| x.split("").map(|x| (x, 1)).collect::<HashMap<&str, u32>>())
    .array_chunks()
    .map(|[l1, l2, l3]| l1.into_iter().chain(l2.into_iter()).chain(l3.into_iter()).fold(HashMap::new(), |mut acc, (key, _)| {
        let val = acc.get(key).unwrap_or(&0);
        acc.insert(key, val + 1);
        acc
      })
    )
    .map(|count_map| {
      let (duplicate, _) = count_map.into_iter().filter(|(x, _)| !x.is_empty()).find(|(_, count)| *count == 3).unwrap();

      char_to_priority(duplicate.chars().nth(0).unwrap())
    })
    .sum()
}