rm -Rf ./game_of_life_engine
rm -Rf ../game_of_life_engine
wasm-pack build --target web --out-dir game_of_life_engine
mv ./game_of_life_engine ../game_of_life_engine
