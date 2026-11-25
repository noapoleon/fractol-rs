# fractol-rs

A fractal explorer written in Rust, using [minifb](https://crates.io/crates/minifb)
for the window. It renders the Mandelbrot set and Julia sets.

## Running

```sh
cargo run --release
```

Use `--release`: escape-time rendering is very slow in debug builds.

## Controls

| input              | action                                              |
|--------------------|-----------------------------------------------------|
| left mouse drag    | pan                                                 |
| mouse wheel        | zoom                                                |
| `W` / `S`          | double / halve `max_iter`                           |
| `F`                | switch between Mandelbrot and Julia                 |
| `C`                | toggle whether the Julia constant `c` follows the mouse |
| mouse move         | sets the Julia `c` while follow mode is on          |
| `Esc`              | quit                                                |

## How it works

- **Input** (`input.rs`): `Input::poll` reads minifb once per frame and turns raw
  events into high-level `Action`s. Panning moves the view in whole pixels; the
  fractional rest is kept in `pan_carry` and added to the next frame.
- **View** (`view.rs`): `Viewpoint` maps screen pixels to points in the complex
  plane (`center`, `scale`), sampling at pixel centers. Uses `f64`.
- **Rendering** (`render.rs`): `Renderer` owns the pixel buffer and the
  `RenderParams` (fractal, `max_iter`, Julia `c`, follow mode). Each change
  returns a `Redraw` (`None`, `Partial(dx, dy)` or `Full`), and these are merged
  per frame:
  - `None`: nothing is recomputed, only `window.update()` runs.
  - `Partial`: after a pan, the still-visible pixels are shifted and only the
    newly exposed strips are computed.
  - `Full`: zoom, `max_iter`, fractal or Julia `c` changed, so every pixel is
    recomputed.

## TODO

### Fixes
- [ ] `pan_delta`: the carry is stored as `dx - fx` instead of `fx - dx`, so
      fractional movement is lost and the view lags behind the cursor
- [ ] Iteration counts are cast to `u8`, which breaks for `max_iter > 255`
      (`W` reaches 256 after three presses). Keep them as `u32` and convert only
      for the final color
- [ ] `change_max_iter`: multiplying can overflow `u32`, so clamp the result
      (dividing is already guarded against 0)
- [ ] `render_partial`: shift the buffer in place (`copy_within`, or iterate in
      the right direction) instead of cloning the whole buffer every pan

### Features
- [x] Julia set, with `c` following the mouse
- [x] Only redraw when something changed
- [x] Partial redraw when panning
- [ ] Zoom with the mouse wheel, centered on the cursor
- [ ] Color palettes instead of a single color ramp
- [ ] Distance estimation (DEM) so thin filaments like the antenna on the
      real axis stay visible
- [ ] Supersampling (several samples per pixel) for smoother edges
- [ ] **Nebulabrot mode**, toggled with a keybind (see below)

## Nebulabrot

The normal render colors each pixel by **where it starts**: how many iterations
the point `c` takes to escape, or black if it never does.

The Buddhabrot and Nebulabrot color the image by **where the points go**:

1. Pick many points `c` (random, or a grid finer than the screen).
2. Iterate `z = z² + c` and keep the orbit (`z₁, z₂, …`).
3. If the point **escapes**, go back over its orbit and add 1 to a hit counter
   for every pixel the orbit passed through. Points that never escape are
   thrown away.
4. Map the hit counts to brightness, normalized by the highest count and
   usually with a sqrt or log curve.

The **Nebulabrot** does this three times with different `max_iter` limits and
puts each result in its own color channel, for example:

| channel | max_iter |
|---------|----------|
| red     | 5000     |
| green   | 500      |
| blue    | 50       |

Planned behavior:
- Press a key (e.g. `N`) to switch between the escape-time view and the Nebulabrot view.
- The Nebulabrot is much slower than escape time, so compute it once per view
  and cache it instead of redrawing every frame.
