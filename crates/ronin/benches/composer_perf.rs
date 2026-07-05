use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ronin::composer::ComposerEditor;

fn fixture(size_kb: usize) -> String {
    let paragraph = "ronin composer benchmark content with markdown-like prose and enough variance to mimic real typing sessions.";
    let mut text = String::with_capacity(size_kb * 1024 + 1024);
    while text.len() < size_kb * 1024 {
        text.push_str(paragraph);
        text.push('\n');
        text.push_str("- list item alpha\n- list item beta\n");
        text.push_str("`inline_code`");
        text.push('\n');
    }
    text
}

fn make_editor(size_kb: usize) -> ComposerEditor {
    let mut editor = ComposerEditor::new();
    editor.set_container_width(420.0);
    editor.set_text(fixture(size_kb));
    editor
}

fn bench_move_down(c: &mut Criterion) {
    let mut group = c.benchmark_group("composer_move_down");

    for size_kb in [1usize, 10, 100] {
        let mut editor = make_editor(size_kb);
        group.bench_with_input(BenchmarkId::from_parameter(format!("{size_kb}kb")), &size_kb, |b, _| {
            b.iter(|| {
                editor.move_down();
                if editor.cursor() == editor.text().len() {
                    editor.select_all();
                    editor.move_left();
                }
                black_box(editor.cursor());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_move_down);
criterion_main!(benches);
