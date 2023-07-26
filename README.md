# PhotoShoop
A Rust server that recursively finds photos in "./" and serves them as a zero-UI slideshow at localhost:4015.

- Shuffles photos, then loops through them
- Transitions between slides every 4 seconds
- Doesn't respond to touch events, except for fullscreen \[F] button
- Can handle iOS live photos (HEIC format)

Motivation: My kids love to look at family photos but can't be trusted with any app that has a "delete" button.

## Dependencies

[Rust](https://www.rust-lang.org/tools/install)

## Run in development mode

`cargo run` or `cargo-watch -x run`.

## Run project

Download repo, add photo folders to project root, then `cargo run -r`.