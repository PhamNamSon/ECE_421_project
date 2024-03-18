use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/lib.rs"]
mod lib;
use lib::red_black_tree::RedBlackTree;

fn red_black_insert(x: i32){
    let mut tree: RedBlackTree<i32> = RedBlackTree::new();
    for i in 0..x{
        tree.insert(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 10k", |b| b.iter(|| red_black_insert(black_box(10000))));
    c.bench_function("insert 40k", |b| b.iter(|| red_black_insert(black_box(40000))));
    c.bench_function("insert 70k", |b| b.iter(|| red_black_insert(black_box(70000))));
    c.bench_function("insert 100k", |b| b.iter(|| red_black_insert(black_box(100000))));
    c.bench_function("insert 130k", |b| b.iter(|| red_black_insert(black_box(130000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);