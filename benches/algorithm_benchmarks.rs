use criterion::{criterion_group, criterion_main, Criterion};

use maze_gen::grids::grid::Grid;

use maze_gen::generation::{
    AldousBroder, Backtracker, BinaryTree, Ellers, Generator, HuntAndKill, Sidewinder, Wilsons,
};

fn benchmark_algorithms(c: &mut Criterion) {
    let grid = Grid::new(100, 100);

    let mut group = c.benchmark_group("Gen");

    group.bench_function("Aldous Broder", |b| {
        b.iter(|| AldousBroder::gen(&mut grid.clone()))
    });

    group.bench_function("Backtracker", |b| {
        b.iter(|| Backtracker::gen(&mut grid.clone()))
    });

    group.bench_function("Binary Tree", |b| {
        b.iter(|| BinaryTree::gen(&mut grid.clone()))
    });

    group.bench_function("Eller's", |b| b.iter(|| Ellers::gen(&mut grid.clone())));

    group.bench_function("Hunt and Kill", |b| {
        b.iter(|| HuntAndKill::gen(&mut grid.clone()))
    });

    group.bench_function("Sidewinder", |b| {
        b.iter(|| Sidewinder::gen(&mut grid.clone()))
    });

    group.bench_function("Wilson's", |b| b.iter(|| Wilsons::gen(&mut grid.clone())));
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);
