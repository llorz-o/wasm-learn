#![feature(test)]

extern crate test;
extern crate wasm_learn;

/*
    cargo install cargo-benchcmp
    注释所有 trick 方法执行时引用的与 wasm-bindgen 相关的代码
    将测试数据写入文件
    cargo bench > control
    cargo bench > variable
    对比之前的文件
    cargo benchcmp control variable

    D:\PERSONAL_WORKSPACE\wasm-learn>cargo benchcmp control variable
    name            control ns/iter  variable ns/iter  diff ns/iter  diff %  speedup
    universe_ticks  655,370          653,440                 -1,930  -0.29%   x 1.00

*/

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_learn::Universe::new();

    b.iter(|| {
        universe.trick();
    })
}