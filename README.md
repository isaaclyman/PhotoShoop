# PhotoShoop
A Rust server that recursively finds photos in "./" and serves them as a zero-UI slideshow at localhost:4015.

- Shuffles photos, then loops through them
- Transitions between slides every 3 seconds
- Doesn't respond to touch events, except for fullscreen button
- Can handle iOS live photos (HEIC format)

Motivation: My kids love to look at family photos but can't be trusted with any app that has a "delete" button.
