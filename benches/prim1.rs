extern crate criterion;
extern crate mintree1;

use criterion::Criterion;

use mintree1::*;

#[test]
fn criterion_benchmark() {
    let graph1 = read_graph_from_file("data1.txt");
    let graph2 = read_graph_from_file("data2.txt");
    let graph3 = read_graph_from_file("data3.txt");
    let graph4 = read_graph_from_file("data4.txt");

    Criterion::default().bench_function_over_inputs(
        "prim1",
        |b, &graph| b.iter(|| prim1::run(&graph, 0)),
        &[&graph1, &graph2, &graph3, &graph4],
    );

    Criterion::default().bench_function_over_inputs(
        "prim2",
        |b, &graph| b.iter(|| prim2::run(&graph, 0)),
        &[&graph1, &graph2, &graph3, &graph4],
    );
}