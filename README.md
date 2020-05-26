# Conban: Console Kanban

Written in rust, made for your productivity. 

Still very much a work in progress, has a couple bugs and inconveniences
, mostly when empty lists are present.
 
Currently, you'll need to install manually by cloning and setting it up how
you like, but I plan to ship a script to auto install in the future.

## Keybindings

You can look at the source [here](https://github.com/almadireddy/conban/blob
/master/src/pane_manager.rs#L138) to edit them on your machine (configurable
 keybindings coming soon!).
 
There are a couple actions which have multiple bindings, and they're shown
 below separated by a vertical bar `|`.
 
- `<l-arrow> | h` : move selection to the pane to the left. 
- `<r-arrow> | l` : move selection to the pane to the right.
- `<u-arrow> | k` : move selection to the item above in the selected list.
- `<d-arrow> | j` : move selection to the item below in the selected list.
- `x` : delete selected item from list
- `X` : delete selected list 
- `i` : insert new item in first list, shows prompt so you can type in the
 name
- `I | +` : insert new list to the end.
- `w/a/s/d` : move the selected item up/down within its list, and left/right
 between lists

## Roadmap

The roadmap is pretty loose at the moment, but current high-priority planned
 features include:
 - reordering lists
 - selection of empty lists
 - inserting items into selected list instead of the first one 