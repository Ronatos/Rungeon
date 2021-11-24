/*
    - Starting areas do not always lead to the correct size passages, only 5ft or 10ft wide ones.
    - Doors shouldn't just lead to 5ft wide passages. There's a whole table that need to be rolled upon to handle that.

    I'm going to build a player, and figure out how to have them traverse between starting areas.
    I think the needs of the player are going to guide the construction of the map. What is the point of an exit if
    there is nobody to go through it? Am I just constructing pretty pictures?
*/

/*
    Player
    
    This one is going to be complex. Where should I start? Well.. the player needs an icon.
    ^, >, v, <

    
*/

mod dice;

mod grid;
use grid::Node as Node;

mod room;

fn main() {

    let room0 = room::starting_area_1::new();
    let room1 = room::starting_area_2::new();
    let room2 = room::starting_area_3::new();
    let room3 = room::starting_area_4::new();
    let room4 = room::starting_area_5::new();
    let room5 = room::starting_area_6::new();
    let room6 = room::starting_area_7::new();
    let room7 = room::starting_area_8::new();
    let room8 = room::starting_area_9::new();
    let room9 = room::starting_area_10::new();
    let room10 = room::starting_area_9::new();
    let room11 = room::starting_area_9::new();

    let map = grid::Grid::new(4, vec![
        // Row 1
        Node::Room(room0),
        Node::Room(room1),
        Node::Room(room2),
        Node::Room(room3),

        // Row 2
        Node::Room(room4),
        Node::Room(room5),
        Node::Room(room6),
        Node::Room(room7),

        // Row 3
        Node::Room(room8),
        Node::Room(room9),
        Node::Room(room10),
        Node::Room(room11),
    ]);

    println!("{}{}{}{}{}{}{}{}{}{}{}{}", map.nodes[0], map.nodes[1], map.nodes[2], map.nodes[3], map.nodes[4], map.nodes[5], map.nodes[6], map.nodes[7], map.nodes[8], map.nodes[9], map.nodes[10], map.nodes[11]);
}