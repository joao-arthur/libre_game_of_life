# libre_game_of_life

A **FLOSS** implementation of _Conway's Game Of Life_.

## Architecture

### lib

A **reusable**, **generic** implementation of _Game of Life_ in _Rust_.

### web_backend

A _Web Assembly_ application that works as a bridge between _lib_ and _web_frontend_, using **wasm-pack**.

### web_frontend

A minimal user application, responsible for render the canvas, render the settings, and init the _Web Assembly_, using **SvelteKit**. Currently, the settings are the following:

- **Preset:** Allows the user to choose from many popular shapes (Glider, Blinker, etc.) 
- **Gap:** A visual gap between the rendered cells
- **Size:** The amount of zoom in the screen
- **FPS:** The desired FPS for render (the actual FPS might be slower)

## Run it

```sh
cd ./web_backend
sh ./build.sh
cd ../web_frontend
pnpm install
pnpm dev
```

## Roadmap

- [ ] Use `setTimeout` over `gloo`
- [ ] Mouse drag
- [ ] More presets
- [ ] Better UI
- [ ] Fix render when resize