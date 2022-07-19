# Tetris game developed in Rust, wasm and React

## Building

Make sure you have [Rust](https://www.rust-lang.org) installed and
[wasm-pack](https://rustwasm.github.io/wasm-pack/). To build this project, run:

``` bash
$> wasm-pack build --target web
```

To run this project, you need a static file server. You can install `sfz` with
cargo:

``` bash
$> cargo install sfz
```

Now, start your static file server and open `index.html`:

``` bash
$> sfz -r
```

## Live demo

[Tetris Live Demo](https://clinquant-halva-a476ee.netlify.app/)
