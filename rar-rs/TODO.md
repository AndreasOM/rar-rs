# TODO


## In Progress


- [ ] Add egui telemetry view

- [ ] Add jump via space bar

- [ ] Add forced slow frame

- [ ] Add tilemap collision
    - [x] Generate tilemap colliders
    - [x] Merge adjacent colliders (2x1, 1x2, 2x2)
    - [ ] Add sideways collision
    - [ ] Prepare optimized getting of potential colliders
    - [+] Add a fixed_update to entities

## TODO

- [ ] Backfill new telemetry tracing to align frames

- [ ] Clip tile rendering on top and bottom

- [ ] Fix font rendering

- [ ] Add support for parallax in map layers. Maybe for decoration layers only


- [ ] Implement and balance basic gameplay mechanics
    - [ ] Run
    - [ ] Jump
    - [ ] Double Jump
    - [ ] Die
    - [ ] Respawn

- [ ] Serialise game progression and settings

- [ ] Add basic inventory to track items
- [ ] Add collectables (-> coins, power ups, etc)
    - [ ] Including uniquely collectable items (via inventory)
- [ ] Work on art style ("look")
    - [ ] Improve placeholder tilesets (v0.2)
    - [ ] Improve placeholder UI (v0.2)

- [ ] Create a test world
- [ ] Implement camera system (with freeze & thaw)
- [ ] Implement basic game states (e.g. Intro, Menu, Game, Credits)
    - [x] Implement game state `game`
    - [x] Implement game state `menu`
    - [x] Implement game state `settings`
    - [ ] Implement game state `intro`
    - [ ] Implement game state `credits`
- [ ] Decide how to model Entity state
- [ ] Autogenerate CRCs in enums, or get rid of them
- [ ] Improve camera
    - [ ] Move world when player gets close to edge

- [ ] Add test case for entity configuration loading from yaml
- [ ] Decide if entities should have components, e.g position
- [ ] Load renderer setup from config file

- [ ] Move filesystem setup boilerplate to oml-game
- [ ] Understand why macOS can load data from .app


- [ ] Experiment with passing the parent UiElement(Container) to all/most/some UiElement(Container?) methods


## TODO - Later
- [ ] Dump all UIElements to yaml config -> do as we go
- [ ] Add support for font selection from ui config for label
- [ ] Improve hotreloading of UI from assets
- [ ] Refactor button click handling
- [ ] Get rid of pointers/handles in UI element/container
- [ ] Reconsider how padding works in the UI
- [ ] Fix padding for hbox & vbox (Note: we might not use these anymore)
- [ ] Trigger `cargo check` on pull request
- [ ] Trigger sound effects from ui button
- [ ] Experiment with speeding up package builds with sccache
- [ ] Allow GameState to cleanly finish when going to next state

## TODO - off stream
- [ ] Push sound blocking down to `oml-audio`
- [ ] Remove dead from entity manager
- [x] Understand how to organise tiled worlds -> see first part of Episode 0026


## Obsolete
- [+] Create generic, reusable UiRoot to remove all the boilerplate -> Nope, this is what UiSystem is supposed to be
- [+] Add basic UI -> Done in other tasks


## Done

### Episode 0037 -> 3.0h + ...
- [x] Add egui debug view
    - [x] Toggle egui debug view via hotkey ("`", grave `)
    - [x] Handle egui debug input
    - [x] Add basic egui debug panel
    - [x] Capture mouse button 0/left while egui is open

### Episode 0036 -> 2.0h + 0.5h ->  89.h    + 75.5h     -> 184.5h
- [x] Add basic, rough player logic

### Episode 0035 -> 3.0h + 15.5h -> 87.0h   + 75.0h     -> 182.0h
- !!! Spend a lot of time on the ScriptContext lifetime issue!
- [x] Add very simplistic scripting
- [x] Add slow motion mode (normal timesteps, but less often) (Use 'u' and 'i' keys)
- [x] Add ScriptVm to run scripts.
- [x] Add ScriptFunctions for wait_frames, and queue_screenshot.
- [x] Add ScriptContext to ScriptFunctions.
- [x] Add ScriptFunction to fake a click at a fixed position.
- [x] Add support for calling fn in the same script.

### Episode 0034 -> 4.0h + 5.0h -> 84.0h    + 59.5h     -> 143.5h
- [x] Add support for generic telemetry traces
- [x] Use improved telemetry
- [x] Add some more colors for debug rendering
- [x] Add simple way to take screenshots, and sequences (slow)
- [x] Add very simplistic script verifier (:WIP: scripting support)
- [x] Add very simplistic script to item parser (:WIP: scripting support)


### Epsiode 0033 -> 3.5h + 1.0h -> 80.5h    + 54.5h     -> 135.0h 
- [x] Add generic telemetry logger :WIP:
- [x] Add (broken) left collision

## Released - on [itch.io](https://omni-mad.itch.io/rar-rs) - password: **rar-rs**

### Episode 0032 - 3.0h + 0.5h  -> 77.0h    + 53.5h     -> 130.5h
- [x] Add very simple debug zoom (via mouse wheel and '/' key)
- [x] Merge adjacent colliders (2x1, 1x2, 2x2)
- [x] Use omt-atlas new autosizing feature to reduce atlas sizes

### Episode 0031 - 3.5h + 2.5h  -> 74.0h    + 53.0h     -> 127.0
- [x] Dump (some) UI config to yaml
- [x] Create world selection via config file
- [x] Create the "world button" from config file
- [x] Add tag reuse, aka duplicated tag support
- [x] Add world selection via menu
- [x] Create the "world button" from config file
- [x] Configure world list via config file
- [x] Display audio backend in settings

### Episode 0030 - 5.0h + 0.0h  ->  70.5h   + 49.5h     -> 120.0
- [x] Allow registering custom UIElements for creation from config ("producer (fn) & factory")
- [x] Fix fading out SettingsDialog when opened in game
- [x] Add hotreloading of UI from assets
- [x] Add clean quit game (with confirmation)

### Episode 0029 - 3.5h + 3.0h  ->  65.5h   + 49.5h     -> 115.0h
- [x] Load UI config from yaml string
- [x] Add confirmation when leaving game
- [x] Fix alignment of ingame pause buttons using grid box
- [x] Handle UI fade when creating from config
- [x] Load ui layout from file, including fallback with error message
- [x] Add support for multiple fonts


### Episode 0028 - 5.0h + 0.5h  ->  62.0h   + 46.5h     -> 108.5h
- [x] Improve layout of settings dialog
- [x] Add support for tagging ui element instances, and finding them
- [x] Add UI tags for easier access of UI elements
- [x] Replace all UI path usage by UI tags
- [x] Create UiGridBox
- [x] Use grid box for settings dialog

### Episode 0027 - special
In episode 0027 we only caught up with the work done over the new years break

### Epsiode 0026 - 2.0h + 6.0h  ->  57.0h   + 46.0h     -> 103.0h
- [x] Add mystic mountain (tiles, tilset, and map) with automapping rules

- [x] Extract settings state into dialog
- [x] Move sound & music button update into settings dialog
- [x] Add background to settings dialog
- [x] Fix text alignment in settings dialog
- [x] Create 3x3 UiImage
- [x] Add some helper to make working with the UI easier
- [x] Add Play/Pause toggle button
- [x] Add Pause menu
    - [x] Add button to open settings
    - [x] Add button to go back to menu
    - [x] Hook up button to open ingame settings
- [x] Add hotkey `=` to cycle through ui debug modes
- [x] Extract/remove `back` button from settings
- [x] Hook up music & sound toggle buttons when settings is used ingame

- [x] Split game & game state
- [x] Fix pause state on game start
- [x] Cleanup ui handling to use newer features/helpers

### Epsiode 0025 - 1.0h + 0.0h  ->  55.0h   + 40.0h     -> 55.0h
- [x] Fix git messup

### Epsiode 0024 - 2.0h + 4.0h  ->  54.0h   + 40.0h     -> 94.0h
- [x] Add music & sound toggle to settings
- [x] Use typed message for sound channel instead of raw strings
- [x] Add grassland tiles, tileset, and test map/world.

### Epsiode 0023 - 2.0h + 0.0h  ->  52.0h   + 36.0h     -> 88.0h
- [x] Add basic sound playback
- [x] Add button sounds :WIP:
- [x] Add sound/music support (via oml-audio)

### Episode 0022 - 2.5h + 2.5h  ->  50.0h   + 36.0h     -> 86.0h
- [x] Add explicit `cargo fetch` to build
- [x] Change the font
- [x] Display build information in game
    - [x] Build Number
        - [x] Code
        - [x] Data
    - [x] Commit Hash
    - [x] Version
    - [x] Date & Time
        - [x] Shorten time display to stop at seconds
    - [+] "Variant"

### Epsiode 0021 - 3.0h + 0.0h  ->  47.5h   + 33.5h     -> 81.0h
- [x] Automate packaging, and releasing
- [x] Package & Upload on tag `-test`
- [x] Add setting state
- [x] Add some build info to settings

### Epsiode 0020 - 3.0h + 0.5h  ->  44.5h   + 33.5h     -> 78.0h
- [x] Package & Upload to Itch

### Epsiode 0019 - 3.0h + 3.0h  ->  41.5h   + 33.0h     -> 75.5h
- [x] Start to cleanup variable handling in github workflows
- [x] Allow using latest data for package
- [x] Load data from .omar

### Episode 0018 - 3.0h + 1.0h  ->  38.5h   + 30.0h     -> 68.5h
- [x] Start to automate releases via github actions

### Epsiode 0017 - 1.5h + 0.0h  ->  35.5h   + 27h       -> 62.5h
- [#] Failed attempt at updating glutin to 0.30.0

### Epsiode 0016 - 2.5h + 2.0h  ->  34.0h   + 27h       -> 61.0h
- [x] Split update into flexible and fixed step
- [x] Remember window position and size across starts

### Epsiode 0015 - 2.0h + 2.0h  ->  31.5h   + 25h       -> 56.5h
- [x] Add basic rectangle rectangle collision test
- [x] Add player world collision
- [x] Pass world through to Player update.

### Episode 0014 - 2.0h + 0.0h  -> 29.5h + 23.0h        -> 52.5h
- [x] Add text to UI system
- [x] Add dummy font
- [x] Add simple button handling for menu
    - [x] Add real UI system (from fiiish-rs)
        - [x] Handle resizing
        - [x] Implement tree like creation of UI hierarchies (aka trees)

### Episode 0013 - 2.0h + 1.5h  -> 27.5h + 23.0h        -> 50.5h
- [x] Add dummy UI buttons

### Epsiode 0012 - 2.0h + 0.0h  -> 25.5h + 21.5h        -> 47h
- [x] Add specific getters for GameState
- [x] Add AppUpdateContext

### Episode 0011 - 3.5h + 4h    -> 23.5h + 21.5h        -> 45h
- [x] Render only visible tiles (x axis check only for now)
- [x] Render tilemap
    - [x] Use correct texture
    - [x] Apply chunk offset
    - [x] Render only visible parts of world
    - [x] Allow per layer configuration of render effect and layer
    - [x] Cleanup visibilty check for chunks


### Episode 0010 - 2.0h + 3.5h  -> 20.0h + 17.5h        -> 37.5h
- [x] Add test helper to 'T'erminate and 'R'espawn player
- [x] Use player spawn position from map
- [x] Use camera start position from map
- [x] Add debug text to map debug overlay
- [x] Add 16 segment debug text rendering
- [x] Fix warnings
- [x] Load tilesets
- [x] Use tile configuration from Tileset for Map rendering
- [x] Allow per layer configuration of render effect and layer

### Episode 0009 - 2.0h + 1h    -> 18.0h + 14.0h        -> 32.0h
- [x] Render some tiles
- [X] Create Tileset loader

### Episode 0008 - 1.5h + 0h    -> 16.0h + 13.0h        -> 29.0h
- [x] Add `%0*[1-8]d` handling to AnimatedTexture
- [x] Implement simple camera FX ("punch it", e.g. for player death)

### Episode 0007 - 2.0h + 1.5h  -> 14.5h + 13.0h        -> 27.5h
- [x] Implement basic player follow camera
- [x] Implement MatrixStack to allowing pushing, popping, and multiplying
- [x] Rework EntityManager to allow retrieving entities by EntityId
- [x] Add per layer translation :HACK:
- [x] Add very basic player follow camera.
- [x] Add fixed debug camera.

### Episode 0006 - 1.5h + 0h    -> 12.5h + 11.5h        -> 24.0h
- [x] Start adding simple camera

### Episode 0005 - 1.5h + 4h    -> 11.0h + 11.5h        -> 22.5h
- [x] Load entity configuration for player from file

### Episode 0004 - 2h + 2h      -> 9.5h + 6.5h          -> 16.0h
- [x] Add background entity
- [x] Add backflip assets
- [x] Implement entity manager, and use it for background and player
- [x] Add input mapping
- [x] Load entity configuration from yaml

### Episode 0003 - 2.0h + 2.0h  -> 7.5h + 4.5h          -> 12.0h
- [x] Start to extract the game state
- [x] Add background assets

### Episode 0002 - 2.0h + 2.5h  -> 5.5h + 2.5h          ->  8.0h
- [x] Add a dummy idle animation
- [x] Add content cooking script
- [x] Add standard render effects


Note: We didn't track progress cleanly before this.

### Episode 0001 - 2.0h + 0h    -> 3.5h + 0.0h          ->  3.5h

 Got something rendering on the screen.


### Episode 0000 - 1.5h + 0h    -> 1.5h + 0.0h          ->  1.5h



~~Note:~~
~~Nothing released yet, but you can run the current version by building the source yourself.~~
