extern crate criterion;
extern crate mintree1;

use criterion::Criterion;

use mintree1::*;

#[test]
fn criterion_benchmark() {
    let data1 = read_graph_from_file("data1.txt");
    let data2 = read_graph_from_file("data2.txt");
    let data3 = read_graph_from_file("data3.txt");

    Criterion::default().bench_function("data 1 algorithm 1", |b| {
        b.iter(|| prim1::run(&data1.0, data1.2, 0))
    });

    Criterion::default().bench_function("data 2 algorithm 1", |b| {
        b.iter(|| prim1::run(&data2.0, data2.2, 0))
    });

    Criterion::default().bench_function("data 3 algorithm 1", |b| {
        b.iter(|| prim1::run(&data3.0, data3.2, 0))
    });
}