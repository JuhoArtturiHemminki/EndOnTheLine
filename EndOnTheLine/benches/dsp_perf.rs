use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use end_on_the_line::process_dsp_parallel;

fn bench_dsp_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("EndOnTheLine_Cache_Test");

    // 1. PIENI DATA (65 536 elementtiä = ~256 kt) - Mahtuu täydellisesti välimuistiin
    let small_size = 65_536;
    let v_small = vec![1.5f32; small_size];
    let mut m_small = vec![0u16; small_size / 16];
    
    group.throughput(Throughput::Elements(small_size as u64));
    group.bench_function("cached_small_stream", |b| {
        b.iter(|| {
            process_dsp_parallel(black_box(&v_small), black_box(&mut m_small));
        })
    });

    // 2. SUURI DATA (1 048 576 elementtiä = ~4 mt) - Joutuu hakemaan RAM-muistista
    let large_size = 1_048_576;
    let v_large = vec![1.5f32; large_size];
    let mut m_large = vec![0u16; large_size / 16];

    group.throughput(Throughput::Elements(large_size as u64));
    group.bench_function("ram_bound_large_stream", |b| {
        b.iter(|| {
            process_dsp_parallel(black_box(&v_large), black_box(&mut m_large));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_dsp_sizes);
criterion_main!(benches);
