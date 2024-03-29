# Nebula Roadmap
---

## 0.1.0
- [X] colors
- [X] attributes
- [X] cursor manipulation
- [X] enter/exit alt screen
- [X] enter/exit raw mode
- [X] scrolling
- [X] events
- [X] input

## Features

### Functional Interfaces
- [X] colors
- [X] attributes
- [X] cursor manipulation (movement, line erasing)
- [X] enter/exit alt screen
- [X] enter/exit raw mode
- [X] scrolling

### Streaming Interfaces
- [X] events (keys, modifiers, etc)
    - [X] parse single event
        - [X] csi events
        - [X] mouse events
        - [X] key events
        - [X] utf8 characters
        - [ ] modifier keys (in control sequences)
        - [ ] extended CSI u sequence
        - [ ] keyboard + device enhancements
- [ ] input
    - [ ] event filters
    - [X] event iterator

## Goals
- [ ] allocation-free
- [ ] async event handling
    - single-threaded for no_std contexts

## API
- [X] central error enum
    - convert errors from other crates
- [ ] unix + windows apis
    - unix first, then windows
- [ ] support checks
    - ansi codes
    - color codes
