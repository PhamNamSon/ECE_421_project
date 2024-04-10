use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/lib.rs"]
mod lib;
// use lib::red_black_tree::RedBlackTree;
use lib::red_black_tree::RBTree;
use lib::avl_tree::AVLTree;


fn red_black_insert(x: u32) -> RBTree{
    let mut tree: RBTree = RBTree::new();
    for i in 0..x{
        tree.insert(i);
    }
    tree
}

fn avl_insert(x: i32) -> AVLTree{
    let mut tree = AVLTree::new();
    for i in 0..x{
        tree.insert_pub(i.into()); 
    }
    tree
}

fn insertion_benchmark(c: &mut Criterion) {
    let insertion_amounts: Vec<u32> = vec![10000, 40000, 70000, 100000, 130000];
    for tree_size in insertion_amounts.iter() {
        let mut bench_group = c.benchmark_group(format!("{} Tree Insertions", tree_size));

        bench_group.bench_function("Red Black Tree", |b| {
            b.iter(||red_black_insert(*tree_size));
        });

        bench_group.bench_function("AVL Tree", |b|{
            b.iter(||avl_insert(*tree_size as i32));
        });
    }
}

fn searches_benchmark(c: &mut Criterion) {
    let insertion_amounts: Vec<u32> = vec![10000, 40000, 70000, 100000, 130000];
    for tree_size in insertion_amounts.iter() {
        let red_black_tree: RBTree = red_black_insert(*tree_size);
        let avl_tree: AVLTree = avl_insert(*tree_size as i32);

        let mut bench_group = c.benchmark_group(format!("{} Tree Searches", tree_size/10));
        bench_group.bench_function("Red Black Tree", |b| {
            b.iter(||{
                for i in 0..*tree_size/10{
                    red_black_tree.search_node(i);
                }
            });
        });

        bench_group.bench_function("AVL Tree", |b| {
            b.iter(||{
                for i in 0..*tree_size/10{
                    avl_tree.search_tree(i.into());
                }
            });
        });
    }
}

criterion_group!(benches, insertion_benchmark, searches_benchmark);
criterion_main!(benches);