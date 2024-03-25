use binary_tree::models::tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new_binary();

    tree.insert(5);
    tree.insert(3);
    tree.insert(8);
    tree.insert(2);
    tree.insert(4);

    tree.print_inorder();
}
