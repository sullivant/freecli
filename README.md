# Free CLI
A Freecell with a CLI interface.

```text
Freecells:   [ K♠] [ K♣] [   ] [ 5♠] Foundations:   [ A♠] [   ] [   ] [ 2♣] 

Columns:
 C0   C1   C2   C3   C4   C5   C6   C7  
 4♥   4♣   7♠   A♦   5♣   6♥   8♠   Q♠  
 8♦   2♠   A♥   Q♦   Q♥  10♣   2♥  10♥  
 6♦   J♥   7♦   8♥   6♣   7♣   J♣   9♣  
 8♣   J♦   4♦   6♠   5♦   3♣   3♠       
 3♦   5♥   K♦   9♠   4♠                 
 9♦   7♥  10♠   2♦   3♥                 
 J♠        9♥   K♥                      
10♦             Q♣                      
```


```text
A Freecell CLI interface.

Usage: freecli [OPTIONS] [POSITIONS]...

Arguments:
  [POSITIONS]...  Possible positions in the format of <from> <to>, ie: "c0 c3" or "c6 foundation" [possible values: c0, c1, c2, c3, c4, c5, c6, c7, f0, f1, f2, f3, foundation]

Options:
      --reset        Reset the game and generate a new board.
      --print        Simply print the current game state.
      --stats        Print more detailed statistics about game wins, attempts, etc.
      --history      Print a history of moves made for this current game.
      --undo         Undo the last move in the game history.
      --seed <SEED>  Optional, if passed will seed the RNG with the value passed for repeatability.
  -h, --help         Print help
  -V, --version      Print version
  ```
  