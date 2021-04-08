// trapping rain water

// https://leetcode.com/problems/trapping-rain-water/

// TODO: this runs in O(n^2) time, make it O(n)
pub fn trapped_water(height: Vec<i32>) -> i32 {
    let mut _max_l = 0;
    let mut total_water = 0;
    let length = height.len();

    if length <= 2 {
        return total_water;
    }

    for i in 1..height.len() - 1 {
        if height[i - 1] > _max_l {
            _max_l = height[i - 1]
        }

        let mut j = i + 1;
        let mut _max_r = 0;
        while j < height.len() {
            if height[j] > _max_r {
                _max_r = height[j]
            }
            j += 1;
        }

        let current_height = std::cmp::min(_max_l, _max_r) - height[i];

        if current_height > 0 {
            total_water += current_height;
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
