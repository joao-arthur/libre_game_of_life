# game-of-life

`canvas` / `opengl` implementation of the conway's game of life.

## Conway's game of life rules

- Any live cell with fewer than two live neighbours dies, as if by
  underpopulation.
- Any live cell with two or three live neighbours lives on to the next
  generation.
- Any live cell with more than three live neighbours dies, as if by
  overpopulation.
- Any dead cell with exactly three live neighbours becomes a live
  cell, as if by reproduction.

## TODO

- unify model and proxy for the sake of simplicity
- tiles size
- click to dead/live
- presets
- change rules
- validations
- opengl
