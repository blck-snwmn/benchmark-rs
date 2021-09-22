#![feature(test)]
extern crate test;

use match_and_map::FnvHashmapMatcher;
use match_and_map::HashmapMatcher;
use match_and_map::MatchMatcher;
use match_and_map::Matcher;
use rand::Rng;

#[bench]
fn match_match_bench(b: &mut test::Bencher) {
    let mather = MatchMatcher::default();
    let mut rng = rand::thread_rng();
    b.iter(|| mather.do_match(test::black_box(rng.gen())))
}

#[bench]
fn match_hashmap_bench(b: &mut test::Bencher) {
    let mather = HashmapMatcher::default();
    let mut rng = rand::thread_rng();
    b.iter(|| mather.do_match(test::black_box(rng.gen())))
}

#[bench]
fn match_hashmap_bench2(b: &mut test::Bencher) {
    let mather = HashmapMatcher::default();
    b.iter(|| mather.do_match(1))
}

#[bench]
fn match_fnv_hashmap_bench(b: &mut test::Bencher) {
    let mather = FnvHashmapMatcher::default();
    let mut rng = rand::thread_rng();
    b.iter(|| mather.do_match(test::black_box(rng.gen())))
}
