# Chess implementation

## Documentation

Full documentation ETA: Soon™

PGN-Replay not fully implemented, need to first fix broken castling (Probably redo the whole thing).

Main.rs contains fully playable game in console, read and understand the main loops for relevant library functions.

Quick rundown of relevant public functions:

### startround(&self):
Generates all possible moves for given boardstate(self) and saves it in Game.moveset field. Returns gamestate, which is either InProgress or Checkmate (other states not yet implemented). 

Since startround is (should be) called at the start of every round, consult game.moveset for available moves. Saved in hashmap structure, with key as a tuple (usize, usize) and value as a Vec<Action>. The Action struct contains 3 fields: which square it moves from, which square it moves to and the movetype (regular, castling etc).

### make_move(&self, Action)
In order to make a move, simply call make_move() with an action. This will also handle passing the turn(Changing game.player).

### Gen_moveset_from_string()
Another move "generator" (actually just consults previously mentioned hashmap) available and takes in a string on the form of algebraic chess notation, but is probably not relevant when making a GUI.