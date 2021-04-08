//  11. Container With Most Water

// Problem:
// https://leetcode.com/problems/container-with-most-water/

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut start_idx = 0;
    let mut end_idx = height.len() - 1;
    let mut max_area = 0;

    while start_idx < end_idx {
        let start_height = height[start_idx] as usize;
        let end_height = height[end_idx] as usize;
        let l = end_idx - start_idx;
        let h;

        // get smaller height
        if start_height < end_height {
            h = start_height;
            // increment if start
            start_idx += 1;
        } else {
            h = end_height;
            // decrement if end
            end_idx -= 1;
        }

        max_area = std::cmp::max(max_area, l * h);
    }
    return max_area as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_by_seven() {
        assert_eq!(49, max_area(vec![8, 6, 2, 5, 4, 8, 3, 7]));
    }
    #[test]
    fn one_by_one() {
        assert_eq!(1, max_area(vec![1, 1]));
    }
    #[test]
    fn four_by_four() {
        assert_eq!(16, max_area(vec![4, 3, 2, 1, 4]));
    }
}
