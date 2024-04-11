use array_range_queries::{fenwick::FenwinkTree, segment::SegmentTree};

fn main() {
    let nums = vec![1, 3, 5, 7, 10];

    println!("Arreglo original: {:?}", nums);

    println!("------- Segment Tree ----------");
    let seg_tree = SegmentTree::new_segment(&nums);

    println!("Suma en el rango [1, 3]: {:?}", seg_tree.query(1, 3));

    println!("------- Frewick Tree----------");
    let mut frenwick_tree = FenwinkTree::new_fenwick_tree(6);
    for (i, &arr) in nums.iter().enumerate() {
        frenwick_tree.update(i + 1, arr);
    }
    println!(
        "Suma en el rango [1, 3]: {}",
        frenwick_tree.query(3) - frenwick_tree.query(0)
    );
}
