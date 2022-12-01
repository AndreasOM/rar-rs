# TODO


## In Progress

- [ ] Automate packaging, and releasing
    - [x] Pack data
    - [ ] Load data from pack
        - [x] Add .omar to layered file system
        - [ ] Move filesystem setup boilerplate to oml-game
        - [ ] Understand why macOS can load data from .app
    - [x] Create github workflows
        - [x] Trigger workflow on push to `-test`
        - [x] Pack data
        - [x] Sign app
        - [x] Upload to itch.io
        - [x] Cleanup variable/setting handling, and use them for ...
            - [x] Packing data
            - [x] Packaging the app
            - [x] Uploading to itch.io
- [x] Setup automated builds on github

- [ ] Add world selection via menu
- [ ] Add tilemap collision
    - [x] Generate tilemap colliders
    - [ ] Prepare optimized getting of potential colliders
    - [ ] Add a fixed_update to entities

- [ ] Display build information in game
    - [x] Build Number
    - [ ] Commit Hash
    - [ ] Version
    - [x] Date & Time
    - [ ] "Variant"

## TODO


- [ ] Work on art style ("look")
- [ ] Create a test world
- [ ] Implement camera system (with freeze & thaw)
- [ ] Add audio/music support (via oml-audio)
- [ ] Add collectables (-> coins, power ups, etc)
- [ ] Implement basic game states (e.g. Intro, Menu, Game, Credits)
    - [x] Implement game state `game`
    - [x] Implement game state `menu`
- [ ] Decide how to model Entity state
- [ ] Autogenerate CRCs in enums, or get rid of them
- [ ] Move world when player gets close to edge
- [ ] Add test case for entity configuration loading from yaml
- [ ] Decide if entities should have components, e.g position
- [ ] Load renderer setup from config file
- [ ] Add basic UI
- [ ] Experiment with speeding up package builds with sccache


## TODO - off stream
- [ ] Remove dead from entity manager
- [ ] Understand how to organise tiled worlds

## Done

### Epsiode 0021 - 3.0h +
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


## Released

Note:
Nothing released yet, but you can run the current version by building the source yourself.
