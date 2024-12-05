#!/bin/bash

name="lsy969999_github_io"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name $name \
  --out-dir dist \
  --target web target/wasm32-unknown-unknown/release/$name.wasm

# wasm build 에의해 생성되는 fetch를 추적가능한 fetch로 바꾸기
# sed -i '' 's/input = fetch(input)/input = bevyProgressiveFetch(input)/' ./wasm/${name}.js

wasm-opt -Oz --output optimized.wasm dist/${name}_bg.wasm
mv optimized.wasm dist/${name}_bg.wasm

# rm -rf ../../static/game/${name}
# cp -r ./wasm ../../static/game/${name}

gzip -c dist/${name}_bg.wasm > dist/${name}_bg.wasm.gz