# TODO


## In Progress

- [ ] Render tilemap
    - [x] Use correct texture
    - [ ] Apply chunk offset
    - [ ] Render only visible parts of world
    - [x] Allow per layer configuration of render effect and layer


## TODO

- [ ] Implement basic game states (e.g. Intro, Menu, Game, Credits)
    - [x] Implement game state `game`
- [ ] Decide how to model Entity state
- [ ] Autogenerate CRCs in enums, or get rid of them
- [ ] Move world when player gets close to edge
- [ ] Add test case for entity configuration loading from yaml
- [ ] Decide if entities should have components, e.g position

## TODO - off stream
- [ ] Remove dead from entity manager

## Done

### Episode 0010 - 2.0h + 3.5h...
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
