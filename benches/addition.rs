#![feature(test)]


#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use learn_rust;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| learn_rust::add_two(2));
    }
}
