#![feature(test)]

extern crate test;
extern crate wasm_game_of_life_sypher7;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_game_of_life_sypher7::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
