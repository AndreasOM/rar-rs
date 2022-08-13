# TODO


## In Progress

- [ ] Implement basic game states (e.g. Intro, Menu, Game, Credits)
    - [x] Implement game state `game`


## TODO
- [ ] Render tilemap
- [ ] Use player spawn position from map
- [ ] Use camera start position from map
- [ ] Decide how to model Entity state
- [ ] Autogenerate CRCs in enums, or get rid of them
- [ ] Move world when player gets close to edge
- [ ] Add test case for entity configuration loading from yaml
- [ ] Decide of entities should have components, e.g position

## TODO - off stream
- [ ] Hook up backflip
- [ ] Remove dead from entity manager


## Done

### Episode 0008 - 1.5h...
- [x] Add `%0*[1-8]d` handling to AnimatedTexture
- [x] Implement simple camera FX ("punch it", e.g. for player death)

### Episode 0007 - 2.0h + 1.5h
- [x] Implement basic player follow camera
- [x] Implement MatrixStack to allowing pushing, popping, and multiplying
- [x] Rework EntityManager to allow retrieving entities by EntityId
- [x] Add per layer translation :HACK:
- [x] Add very basic player follow camera.
- [x] Add fixed debug camera.

### Episode 0006 - 1.5h + 0h 
- [x] Start adding simple camera

### Episode 0005 - 1.5h + 4h
- [x] Load entity configuration for player from file

### Episode 0004 - 2h + 2h
- [x] Add background entity
- [x] Add backflip assets
- [x] Implement entity manager, and use it for background and player
- [x] Add input mapping
- [x] Load entity configuration from yaml

### Episode 0003 - 2.0h + 2.0h
- [x] Start to extract the game state
- [x] Add background assets

### Episode 0002 - 2.0h + 2.5h
- [x] Add a dummy idle animation
- [x] Add content cooking script
- [x] Add standard render effects


Note: We didn't track progress cleanly before this.

### Episode 0001 - 2.0h

 Got something rendering on the screen.


### Episode 0000 - 1.5h


## Released

Note:
Nothing released yet, but you can run the current version by building the source yourself.
