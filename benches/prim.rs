extern crate criterion;
extern crate mintree1;

use criterion::Criterion;
use criterion::Fun;

use mintree1::*;

#[test]
fn function_benchmark() {
    let graph1 = read_graph_from_file("data1.txt");
    let graph2 = read_graph_from_file("data2.txt");
    let graph3 = read_graph_from_file("data3.txt");
    let graph4 = read_graph_from_file("data4.txt");

    let prim_1 = Fun::new("Prim 1", |b, graph| b.iter(|| prim1::run(&graph, 0)));
    let prim_2 = Fun::new("Prim 2", |b, graph| b.iter(|| prim2::run(&graph, 0)));
    let prim_3 = Fun::new("Prim 3", |b, graph| b.iter(|| prim3::run(&graph, 0)));
    let prim_4 = Fun::new("Prim 4", |b, graph| b.iter(|| prim4::run(&graph, 0)));

    let functions = vec![prim_1, prim_2, prim_3, prim_4];

    Criterion::default().bench_functions("data 1", functions, &graph1);
    // Criterion::default().bench_functions("data 2", functions, &graph2);
    // Criterion::default().bench_functions("data 3", functions, &graph3);
    // Criterion::default().bench_functions("data 4", functions, &graph4);
}

#[test]
fn data_benchmark() {
    let graph1 = read_graph_from_file("data1.txt");
    let graph2 = read_graph_from_file("data2.txt");
    let graph3 = read_graph_from_file("data3.txt");
    let graph4 = read_graph_from_file("data4.txt");

    let graphs = [&graph1, &graph2, &graph3, &graph4];

    Criterion::default().bench_function_over_inputs(
        "prim1",
        |b, &graph| b.iter(|| prim1::run(&graph, 0)),
        &graphs,
    );

    Criterion::default().bench_function_over_inputs(
        "prim2",
        |b, &graph| b.iter(|| prim2::run(&graph, 0)),
        &graphs,
    );

    Criterion::default().bench_function_over_inputs(
        "prim3",
        |b, &graph| b.iter(|| prim3::run(&graph, 0)),
        &graphs,
    );

    Criterion::default().bench_function_over_inputs(
        "prim4",
        |b, &graph| b.iter(|| prim4::run(&graph, 0)),
        &graphs,
    );
}