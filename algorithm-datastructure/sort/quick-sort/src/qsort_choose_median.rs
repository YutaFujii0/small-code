pub fn quick_sort_choose_median(nums: &mut [i64]) -> usize {
  if nums.len() <= 1 {
      return 0
  }

  // swap median element with first element
  let median = (nums.len() - 1) / 2;
  let pivot_index;
  let comp1 =  nums[0] < nums[median];
  let comp2 =  nums[0] < nums[nums.len() - 1];
  let comp3 =  nums[median] < nums[nums.len() - 1];
  match (comp1, comp2, comp3) {
    (true, true, true) => pivot_index = median,
    (false, false, false) => pivot_index = median,
    (true, true, false) => pivot_index = nums.len() - 1,
    (false, false, true) => pivot_index = nums.len() - 1,
    _ => pivot_index = 0,
  }

  let tmp = nums[pivot_index];
  nums[pivot_index] = nums[0];
  nums[0] = tmp;

  let pivot = nums[0];
  let mut i = 1;
  let mut j = 1;
  let comparisons = nums.len() - 1;
  // compare
  while j < nums.len() {
      if nums[j] < pivot {
          let tmp = nums[j];
          nums[j] = nums[i];
          nums[i] = tmp;
          i += 1;
      }
      j += 1;
  }
  i -= 1;
  let tmp = nums[i];
  nums[i] = nums[0];
  nums[0] = tmp;

  // split
  let comps_left = quick_sort_choose_median(&mut nums[..i]);
  let comps_right = quick_sort_choose_median(&mut nums[i+1..]);

  comparisons + comps_left + comps_right
}
