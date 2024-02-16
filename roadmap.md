# Nebula Roadmap
---

## Features

### Functional Interfaces
- [X] colors
- [X] attributes
- [X] cursor manipulation (movement, line erasing)
- [X] enter/exit alt screen
- [X] enter/exit raw mode
- [X] scrolling

### Streaming Interfaces
- [ ] events (keys, modifiers, etc)
- [ ] input

## Goals
- [ ] allocation-free
- [ ] async event handling
    - single-threaded for no_std contexts

## API
- [ ] central error enum
    - convert errors from other crates
- [ ] unix + windows apis
    - unix first, then windows
