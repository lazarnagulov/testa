use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::{collections::HashMap, fs, path::PathBuf};
use testa_core::{analyser::SemanticAnalyser, lexer::Lexer, parser::Parser};
use testa_hir::AstLowering;

fn bench_compiler_pipeline(c: &mut Criterion) {
    let sizes = [1_000, 10_000, 50_000, 100_000, 150_000];
    let mut group = c.benchmark_group("Compiler_Pipeline_Stages");

    for size in sizes {
        let file_path = format!("../testa-lsp/benches/fixtures/fixture_{}.testa", size);
        let path = PathBuf::from(&file_path);

        if !path.exists() {
            continue;
        }

        let content = fs::read_to_string(&path).expect("Failed to read fixture");

        group.bench_with_input(BenchmarkId::new("Parsing", size), &content, |b, src| {
            b.iter(|| {
                let lexer = Lexer::new(src);
                let mut parser = Parser::new(lexer, src);
                let program = parser.parse().unwrap();
                black_box(program);
            });
        });

        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer, &content);
        let program = parser.parse().unwrap();

        group.bench_with_input(
            BenchmarkId::new("Semantic_Analysis", size),
            &program,
            |b, prog| {
                b.iter(|| {
                    let analysis = SemanticAnalyser::new(prog).analyse_with_imports(&[]);
                    black_box(analysis);
                });
            },
        );

        let analysis = SemanticAnalyser::new(&program).analyse_with_imports(&[]);

        group.bench_with_input(
            BenchmarkId::new("Ast_Lowering", size),
            &(&program, &analysis),
            |b, (prog, an)| {
                b.iter(|| {
                    let module =
                        AstLowering::new(path.clone()).lower(prog, an, &content, &HashMap::new());
                    black_box(module);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_compiler_pipeline);
criterion_main!(benches);
