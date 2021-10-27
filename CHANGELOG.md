# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Plan a level editor
- Reorganize where some long term files reside in the tree structure
- calls to spawn_level has a last argument not dependent on NUM_MONSTERS
- Save level function
- Save game function
- Up a level function
- Understand ron crate with respect to crashes when .ron file has a wrong field
- Need scaling factor between HUD layer and Map layer
- In inventory list combine like items and enumerate
- using Dungeon Map revealed wrong theme?
- improve start up time, remove map generation before Begin Game Screen
- Address Warnings

## [0.1.3]

## added

- List equipped items
- moved hud_display_main_menu call to hud::display_main_menu
- Exit Game option
- enlarge font for HUD+
- Combined State/victory and State/game_over into one function in main.rs, State/game_over
- Play Game Screen
- Fix EmptyArchitect warning
- Adjust tooltip draw location for larger HUD font set
- Implement Esc( Pause) and then Continue current game
- get rid of State in display_main_menu by add_system(display_main_menu)??
- move display_cave call to main and reset_game_state

## [0.1.2] - 2021-09-13

## Added

- Material from Chapters 15 and 16

## [0.1.1] - 2021-09-10

### Added

- Material for Chapter 14
- Move to legion 0.4.0
- Moved to Rust 1.55
- Fix startup problem of newlines to start rendering the screen when printing the map to console

## [0.1.0] - 2021-09-09

### Added

- All Material for Hands on Rust: Herbert Wolverson through Chapter 13
- created display_cave from Wolversons example into test_harness directory which is the begging of a test
  harness
