use std::cmp::Ordering;

struct SegmentTree {
    size: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        SegmentTree {
            size,
            tree: vec![0; size],
        }
    }

    fn update(&mut self, mut idx: usize, value: i64) {
        idx += self.size / 2;
        self.tree[idx] = value;
        while idx > 1 {
            idx /= 2;
            self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> i64 {
        l += self.size / 2;
        r += self.size / 2;
        let mut sum = 0;
        while l <= r {
            if l % 2 == 1 {
                sum += self.tree[l];
                l += 1;
            }
            if r % 2 == 0 {
                sum += self.tree[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        sum
    }
}

fn break_even_point(fixed_costs: i64, costs: &Vec<i64>, revenues: &Vec<i64>) -> Option<usize> {
    let n = costs.len();
    let mut cost_tree = SegmentTree::new(n);
    let mut revenue_tree = SegmentTree::new(n);
    
    for i in 0..n {
        cost_tree.update(i, costs[i]);
        revenue_tree.update(i, revenues[i]);
    }
    
    for i in 0..n {
        let total_cost = fixed_costs + cost_tree.query(0, i);
        let total_revenue = revenue_tree.query(0, i);
        if total_revenue >= total_cost {
            return Some(i);
        }
    }
    None
}

fn main() {
    let fixed_costs = 1000;
    let costs = vec![200, 300, 400, 500, 600];
    let revenues = vec![100, 400, 700, 1100, 1500];
    
    match break_even_point(fixed_costs, &costs, &revenues) {
        Some(index) => println!("Break-even point at item: {}", index + 1),
        None => println!("No break-even point found"),
    }
}
