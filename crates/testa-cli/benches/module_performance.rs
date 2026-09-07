use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::path::Path;
use testa_hir::module::resolver::ModuleResolver;

const SET_SIZES: [i32; 5] = [10, 25, 50, 75, 100];

fn bench_resolver_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModuleResolver_Wide_Topology");

    for size in SET_SIZES.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
            let fixture_dir = Path::new(".")
                .join("benches")
                .join("benches")
                .join("fixtures")
                .join(format!("wide_{}", size));

            b.iter_batched(
                || ModuleResolver::new(vec![fixture_dir.clone()]),
                |mut resolver| {
                    resolver
                        .resolve_all(&["main".to_string()], &fixture_dir.join("main.tmod"))
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_resolver_chain(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModuleResolver_Chain_Topology");

    for size in SET_SIZES.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
            let fixture_dir = Path::new(".")
                .join("benches")
                .join("benches")
                .join("fixtures")
                .join(format!("chain_{}", size));

            b.iter_batched(
                || ModuleResolver::new(vec![fixture_dir.clone()]),
                |mut resolver| {
                    resolver
                        .resolve_all(&["main".to_string()], &fixture_dir.join("main.tmod"))
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_resolver_monolith(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModuleResolver_Monolith_Baseline");

    for size in SET_SIZES.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
            let fixture_dir = Path::new(".")
                .join("benches")
                .join("benches")
                .join("fixtures")
                .join(format!("monolith_{}", size));

            b.iter_batched(
                || ModuleResolver::new(vec![fixture_dir.clone()]),
                |mut resolver| {
                    resolver
                        .resolve_all(&["main".to_string()], &fixture_dir.join("main.tmod"))
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_resolver_wide,
    bench_resolver_chain,
    bench_resolver_monolith
);
criterion_main!(benches);
