# rungeon
A Dungeons &amp; Dragons 5th Edition dungeon generator written in Rust.

## To do
1. Starting areas do not always lead to the correct size passages, only 5ft or 10ft wide ones (crate::map::room:place_passage() & starting_areas_1-10.rs should be edited)
2. Established passage width in starting_area needs to be passed out to the map so correctly sized passages can be generated to extend from them (exit object? can other exit types have widths?)
3. Passages need to be built out so map generation can continue
4. Map generation will continue until we eventually learn how chambers will be implemented