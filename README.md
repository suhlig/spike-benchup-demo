# spike-benchup-demo

Two small programs that drive [BenchUp](https://benchup.io/)'s geometry kernel
(`benchup-core`) directly, without the editor UI.

They were reconstructed from the video
[BenchUp: A SketchUp Alternative: how push/pull modeling works](https://youtu.be/-_PAlXlYgik)
by Shekhar Gulati. The video's demo sources were not published; these are
independent reconstructions that use the public `benchup-core` API and reproduce
the output shown on screen.

## About the vendored kernel

The programs target BenchUp tag `v0.6.6` (commit `f4c0f496`). The original
repository, `BenchUp3D/benchup` (formerly `BenchUp3D/editor`), is no longer
publicly reachable, so the `benchup-core` crate and the English message catalog
it generates from are vendored under `vendor/` and `assets/`.

BenchUp is MIT licensed; its license is preserved at
`vendor/benchup-core/LICENSE`.

## Requirements

- Rust 1.95 or newer (edition 2024)

## `faces`

Draws a 60 x 40 cm rectangle one edge at a time, then adds a line across it, and
prints the vertex/edge/face counts plus the resulting faces and their areas.

```sh
cargo run --release --bin faces
```

## `pushpull`

Pulls the rectangle into a box, then runs three further push/pull experiments and
prints vertex/edge/face counts, holes, Euler characteristic, face height, and
solid volume. It also writes `results.json` (or `/work/results.json` when
`/work` exists).

```sh
cargo run --release --bin pushpull
```

## Layout

- `src/bin/faces.rs`
- `src/bin/pushpull.rs`
- `vendor/benchup-core/`
