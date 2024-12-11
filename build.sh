#!/bin/bash

name="lsy969999_github_io"

cargo build --release --no-default-features --target wasm32-unknown-unknown
wasm-bindgen --out-name $name \
  --out-dir dist2 \
  --target web target/wasm32-unknown-unknown/release/$name.wasm

# wasm build 에의해 생성되는 fetch를 추적가능한 fetch로 바꾸기
#sed -i 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' dist2/${name}.js

if [ "$(uname)" = "Darwin" ]; then
  sed -i '' 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' dist2/${name}.js
else
  sed -i 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' dist2/${name}.js
fi

echo "`ls -al`"
echo "`ls -al dist2`"
wasm-opt -Oz --output optimized.wasm dist2/${name}_bg.wasm
mv optimized.wasm dist2/${name}_bg.wasm
echo "`ls -al dist2`"
# echo $(stat -f %z dist/${name}_bg.wasm)