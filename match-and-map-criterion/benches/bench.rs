use match_and_map::FnvHashmapMatcher;
use match_and_map::HashmapMatcher;
use match_and_map::MatchMatcher;
use match_and_map::Matcher;
use rand::Rng;

fn match_match_bench(c: &mut criterion::Criterion) {
    let mather = MatchMatcher::default();
    let mut rng = rand::thread_rng();
    c.bench_function("match_match_bench", |b| {
        b.iter(|| mather.do_match(rng.gen()))
    });
}

fn match_hashmap_bench(c: &mut criterion::Criterion) {
    let mather = HashmapMatcher::default();
    let mut rng = rand::thread_rng();
    c.bench_function("match_hashmap_bench", |b| {
        b.iter(|| mather.do_match(rng.gen()))
    });
}

fn match_fnv_hashmap_bench(c: &mut criterion::Criterion) {
    let mather = FnvHashmapMatcher::default();
    let mut rng = rand::thread_rng();
    c.bench_function("match_fnv_hashmap_bench", |b| {
        b.iter(|| mather.do_match(rng.gen()))
    });
}

criterion::criterion_group!(
    benches,
    match_match_bench,
    match_hashmap_bench,
    match_fnv_hashmap_bench
);
criterion::criterion_main!(benches);
