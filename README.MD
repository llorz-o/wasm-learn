## command

`wasm-pack build`
`wasm-pack test --firefox`

## documentation

[rust-wasm](https://rustwasm.github.io/docs/book/introduction.html)
[rust-wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)


## Exercises
    
- At this point, the next lowest hanging fruit for speeding up Universe::tick is removing the allocation and free. Implement double buffering of cells, where the Universe maintains two vectors, never frees either of them, and never allocates new buffers in tick.

- Implement the alternative, delta-based design from the "Implementing Life" chapter, where the Rust code returns a list of cells that changed states to JavaScript. Does this make rendering to <canvas> faster? Can you implement this design without allocating a new list of deltas on every tick?

- As our profiling has shown us, 2D <canvas> rendering is not particularly fast. Replace the 2D canvas renderer with a WebGL renderer. How much faster is the WebGL version? How large can you make the universe before WebGL rendering is a bottleneck?