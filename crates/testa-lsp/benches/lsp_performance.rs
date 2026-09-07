use criterion::{Criterion, black_box, criterion_group, criterion_main};
use testa_lsp::lsp::backend::Backend;
use tower_lsp::{LanguageServer, LspService, lsp_types::*};

async fn pre_load_document(backend: &Backend, uri: Url, content: String) {
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri,
            language_id: "testa".to_string(),
            version: 1,
            text: content,
        },
    };
    backend.did_open(params).await;
}

async fn bench_did_change(backend: &Backend, uri: Url, text: String) {
    let params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri, version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text,
        }],
    };
    LanguageServer::did_change(backend, params).await;
}

async fn bench_hover(backend: &Backend, uri: Url) {
    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 9,
                character: 17,
            },
        },
        work_done_progress_params: Default::default(),
    };
    let _ = LanguageServer::hover(backend, params).await;
}

async fn bench_goto_definition(backend: &Backend, uri: Url) {
    let params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 9,
                character: 17,
            },
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };
    let _ = LanguageServer::goto_definition(backend, params).await;
}

async fn bench_references(backend: &Backend, uri: Url) {
    let params = ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 9,
                character: 17,
            },
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: ReferenceContext {
            include_declaration: true,
        },
    };
    let _ = LanguageServer::references(backend, params).await;
}

async fn bench_completion(backend: &Backend, uri: Url) {
    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 10,
                character: 4,
            },
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };
    let _ = LanguageServer::completion(backend, params).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (service, _) = LspService::new(Backend::new);
    let backend = service.inner();

    let sizes = [1_000, 10_000, 50_000, 100_000, 150_000];

    for size in sizes {
        let file_path = format!("benches/fixtures/fixture_{}.testa", size);
        let content = std::fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Missing {}. Run your generator script first!", file_path));

        let uri = Url::parse(&format!("file:///benchmark_{}.testa", size)).unwrap();

        rt.block_on(pre_load_document(backend, uri.clone(), content.clone()));

        let mut group = c.benchmark_group(format!("LSP_Scale_{}k", size / 1000));

        group.bench_function("did_change_full_reparse", |b| {
            b.to_async(&rt).iter(|| {
                bench_did_change(black_box(backend), uri.clone(), black_box(content.clone()))
            });
        });

        group.bench_function("hover", |b| {
            b.to_async(&rt)
                .iter(|| bench_hover(black_box(backend), uri.clone()))
        });

        group.bench_function("goto_definition", |b| {
            b.to_async(&rt)
                .iter(|| bench_goto_definition(black_box(backend), uri.clone()))
        });

        group.bench_function("references", |b| {
            b.to_async(&rt)
                .iter(|| bench_references(black_box(backend), uri.clone()))
        });

        group.bench_function("completion", |b| {
            b.to_async(&rt)
                .iter(|| bench_completion(black_box(backend), uri.clone()))
        });

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
