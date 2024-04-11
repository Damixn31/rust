#[derive(Debug)]
pub struct SegmentTree {
    pub tree: Vec<i32>,
    pub n: usize,
}

impl SegmentTree {
    pub fn new_segment(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut tree = vec![0; 2 * n];

        // contruir el arbol de segmento recursivo
        Self::build_tree(nums, &mut tree, 0, 0, n - 1);
        SegmentTree { tree, n }
    }
    pub fn build_tree(
        nums: &[i32],
        tree: &mut [i32],
        idx: usize,
        left: usize,
        right: usize,
    ) -> i32 {
        if left == right {
            tree[idx] = nums[left];
            return nums[left];
        }

        let mid = left + (right - left) / 2;
        let left_sum = Self::build_tree(nums, tree, 2 * idx + 1, left, mid);
        let right_sum = Self::build_tree(nums, tree, 2 * idx + 2, mid + 1, right);

        tree[idx] = left_sum + right_sum;
        tree[idx]
    }

    pub fn query_helper(
        &self,
        idx: usize,
        segment_left: usize,
        segment_right: usize,
        query_left: usize,
        query_right: usize,
    ) -> i32 {
        if query_left <= segment_left && query_right >= segment_right {
            return self.tree[idx];
        }

        if query_right < segment_left || query_left > segment_right {
            return 0;
        }

        let mid = segment_left + (segment_right - segment_left) / 2;
        let left_sum = self.query_helper(2 * idx + 1, segment_left, mid, query_left, query_right);
        let right_sum =
            self.query_helper(2 * idx + 2, mid + 1, segment_right, query_left, query_right);
        left_sum + right_sum
    }

    pub fn query(&self, query_left: usize, query_right: usize) -> i32 {
        self.query_helper(0, 0, self.n - 1, query_left, query_right)
    }
}
