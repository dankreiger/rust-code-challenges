// trapping rain water

// https://leetcode.com/problems/trapping-rain-water/

pub fn trapped_water(height: Vec<i32>) -> i32 {
    let mut total_water = 0;
    let length = height.len();

    // no water possible if 2 or less items
    if length <= 2 {
        return total_water;
    }

    let mut l_idx = 0;
    let mut r_idx = length - 1;
    let mut pointer_idx = l_idx;

    let mut l_max = height[l_idx];
    let mut r_max = height[r_idx];

    while l_idx < r_idx {
        let current_height: i32 = std::cmp::min(l_max, r_max) - height[pointer_idx];

        if height[l_idx] > l_max {
            l_max = height[l_idx]
        }
        if height[r_idx] > r_max {
            r_max = height[r_idx]
        }

        if current_height > 0 {
            total_water += current_height;
        }

        if l_max < r_max {
            l_idx += 1;
            pointer_idx = l_idx
        } else {
            r_idx -= 1;
            pointer_idx = r_idx
        }
    }

    return total_water;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(trapped_water(vec![0, 1, 0, 2, 1, 0, 3, 1, 0, 1, 2]), 8);
    }
    #[test]
    fn it_works2() {
        assert_eq!(trapped_water(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
