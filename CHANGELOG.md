# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.2.0] - 2023-09-19

### Fixed
- Fixed some rounding errors in GUI backend character buffer display math.

### Added
- Storage abstraction for files that uses Web Storage API on WASM builds.
- Recognize horizontal mouse scrolling.

### Changed
- The main loop is now async-based, which hopefully simplifies writing program
  logic. The way miniquad's runtime loop is implemented makes it necessary for
  the user code to be state machine like, and async is the nicest way to hide
  this. The runtime interface is simplified to be based on standalone
  functions that access a singleton.
- Font and palette configuration happen at runtime, removed the configuration
  structure used to start the program.
- Character resolution query takes a preferred size so that text can be
  automatically enlarged on high-DPI displays when using the GUI backend.
- Color quantization into Xterm256 color is much faster but somewhat lower
  quality now.
- Window borders are cleared using the background color in the configuration
  palette in GUI backend.
- Non-printable key names are abbreviated so they can fit better in on-screen
  help text with limited space.

## [0.1.0] - 2023-06-26

Initial release.
