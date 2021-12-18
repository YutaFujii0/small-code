pub fn quick_sort_choose_last(nums: &mut [i64]) -> usize {
  if nums.len() <= 1 {
      return 0
  }

  // swap last element with first element
  let tmp = nums[nums.len() - 1];
  nums[nums.len() - 1] = nums[0];
  nums[0] = tmp;

  let pivot_index = 0;
  let pivot = nums[pivot_index];
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
  nums[i] = nums[pivot_index];
  nums[pivot_index] = tmp;
  // split
  let comps_left = quick_sort_choose_last(&mut nums[..i]);
  let comps_right = quick_sort_choose_last(&mut nums[i+1..]);

  comparisons + comps_left + comps_right
}
