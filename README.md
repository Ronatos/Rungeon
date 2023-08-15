# rungeon
A Dungeons &amp; Dragons 5th Edition dungeon generator written in Rust.

## To do
* Starting areas do not always lead to the correct size passages, only 5ft or 10ft wide ones. Should also include 20, 30, and 40 (with variety)
* Passages and Chambers need to be built out

At this point, we need to create the passages and chambers which will be linked together. There are three problems I see, though.
1. Passage width data is obscured to the map
   1. Rooms should have optional parameters. If they have none, everything is random. If they have some, passage widths and locations can be specified.
2. Passage/door locations are obscured to the map
3. The size of the rooms are asymmetric, which will lead to a lot of display weirdness
   1. The first thought would be to standardize the displayed room sizes around the largest possible room, but this creates the problem of passages being far too long.
   2. Need to prototype out some drawings and see what looks best
   3. I wonder if we could decompose Room Nodes and recompile them as a singular Map Tile Grid. That's sort of what we're looking to do anyways, isn't it? Display the whole thing all at once?
      1. Is there a benefit to doing this over keeping the structures as they are and displaying them continguously instead?