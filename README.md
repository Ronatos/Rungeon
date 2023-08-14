# rungeon
A Dungeons &amp; Dragons 5th Edition dungeon generator written in Rust.

## To do
* Starting areas do not always lead to the correct size passages, only 5ft or 10ft wide ones. Should also include 20, 30, and 40 (with variety)
* Passages and Chambers need to be built out
* Exits need to be worked out and established

## Rooms, Grids, Nodes, and Tiles - How do they work?
A Tile is the physical thing that is displayed when you're walking around in the game. Tiles have types (wall, floor, etc), and they can hold things like a goblin or a treasure chest. If a Tile is holding these things, the things on the Tile are displayed in-game instead of the Tile itself.

Grids are powerful, reusable containers we use to group Tiles together into what looks like an explorable environment in the game. You can actually have Grids of Grids, allowing us to use Grids to model out any location to any scale we want! To achieve this modularity of Grids, we need to utilize Nodes.

Nodes are wrappers for Tiles and Rooms within the context of a Grid. Grids are *actually* made of Nodes, and Nodes can be of type Tile or of type Room. If a Grid is made up of Tile Nodes, that Grid is literally an explorable room. You can interact with each Tile in that room. If a Grid is made up of Room Nodes, that Grid is a map of other Grids.

For example, if you were trying to explore a dungeon, you'd need to transition between rooms in the dungeon. It's not just a big cavern, there are doorways to other rooms. Each Room is a Grid, and is also packaged up into a Node on a *different* Grid, often called something like a Dungeon_Map. That Dungeon_Map is a Grid of Room Nodes, and each Room Node is a Grid of Tile Nodes.

### So how do I use them in practice?

It all starts with creating the layout for a room you want to display. These could be starting areas, passages, or chambers. You'll generate the object like this:
```
let my_room: Grid = room::chamber_67::new();
```

Once your room is created, you'll probably want to create a few more to explore. The first room you created probably has exits, after all. Let's say you've set up several rooms.
```
let room0: Grid = room::chamber_67::new();
let room1: Grid = room::chamber_52::new();
let room2: Grid = room::chamber_01::new();
let room3: Grid = room::passage_04::new();
let room4: Grid = room::starting_area_1::new();
let room5: Grid = room::chamber_10::new();
let room6: Grid = room::chamber_05::new();
let room7: Grid = room::passage_01::new();
let room8: Grid = room::chamber_02::new();
```

How do you create a map of your standalone rooms?
```
let map = grid::Grid::new(3, vec![
        Node::Room(room0), Node::Room(room1), Node::Room(room2),
        Node::Room(room3), Node::Room(room4), Node::Room(room5),
        Node::Room(room6), Node::Room(room7), Node::Room(room8),
    ]);
```

Ta-da! You now have a 3x3 grid of Room Nodes. Each of those Room Nodes is itself a Grid of Tiles.
