struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut ptr1 = 0 as usize;
        let mut ptr2 = 0 as usize;
        let mut merged: Vec<i32> = vec![];
        while ptr1 < len1 || ptr2 < len2 {
            if ptr1 < len1 && ptr2 >= len2 {
                merged.push(nums1[ptr1]);
                ptr1 += 1;
            } else if ptr1 >= len1 && ptr2 < len2 {
                merged.push(nums2[ptr2]);
                ptr2 += 1;
            } else if nums1[ptr1] <= nums2[ptr2] {
                merged.push(nums1[ptr1]);
                ptr1 += 1;
            } else {
                merged.push(nums2[ptr2]);
                ptr2 += 1;
            }
        }
        let total_len = len1 + len2;
        let reminder = total_len % 2;
        match reminder {
            0 => (merged[total_len / 2 - 1] + merged[total_len / 2]) as f64 / 2f64,
            _ => merged[total_len / 2] as f64,
        }
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Result: {}", result);
}
