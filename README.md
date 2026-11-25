# fractol-rs

A fractal explorer written in Rust, starting with the Mandelbrot set.

## TODO

### Fixes
- [ ] Update the `pixel_to_point` tests for pixel-center sampling (expect the
      half-pixel offset, compare floats with a tolerance)
- [ ] `pan`: the `y_offset` debug print shows `x_offset`

### Features
- [ ] Julia set (`julia` exists but isn't wired up yet)
- [ ] Zoom with the mouse wheel, centered on the cursor
- [ ] Color palettes instead of grayscale
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
