# WebGL Basic Water Tutorial

If you have any questions or run into any stumbling blocks please feel free to
[open an issue](https://github.com/chinedufn/webgl-water-tutorial/issues)!

[Read the tutorial](http://chinedufn.com/3d-webgl-basic-water-tutorial/)

![Screenshot of tutorial](/screenshot.png)

```sh
# You can use any static file server that properly sets the
# `application/wasm` mime type
cargo install https

git clone https://github.com/chinedufn/webgl-water-tutorial
cd webgl-water-tutorial

# A version of Rust that can compile wasm-bindgen-cli version 0.2.29
rustup override set nightly-2021-02-11
cargo +nightly-2020-06-22 install -f wasm-bindgen-cli --version 0.2.29 # Or download a release binary

# Build
./build.sh

## Opens your browser to http://localhost:8080  where the demo will be running
http -m wasm:application/wasm
```

# See Also

- [ThinMatrix's OpenGL Water Tutorial](https://www.youtube.com/watch?v=HusvGeEDU_U&list=PLRIWtICgwaX23jiqVByUs0bqhnalNTNZh) - Heavily inspired this WebGL implementation
- [Landon](https://github.com/chinedufn/landon) - Used for exporting meshes and armatures from Blender
