/*
    - Starting areas do not always lead to the correct size passages, only 5ft or 10ft wide ones. Should also include 20, 30, and 40 (with variety)
*/

mod dice;

mod grid;
use grid::Node as Node;
use grid::room as room;
use room::starting_area as starting_area;

fn main() {

    let room0 = starting_area::starting_area_1::new();
    let room1 = starting_area::starting_area_2::new();
    let room2 = starting_area::starting_area_3::new();
    let room3 = starting_area::starting_area_4::new();
    let room4 = starting_area::starting_area_5::new();
    let room5 = starting_area::starting_area_6::new();
    let room6 = starting_area::starting_area_7::new();
    let room7 = starting_area::starting_area_8::new();
    let room8 = starting_area::starting_area_9::new();
    let room9 = starting_area::starting_area_10::new();
    let room10 = starting_area::starting_area_9::new();
    let room11 = starting_area::starting_area_9::new();

    let map = grid::Grid::new(4, vec![
        Node::Room(room0), Node::Room(room1), Node::Room(room2), Node::Room(room3),
        Node::Room(room4), Node::Room(room5), Node::Room(room6), Node::Room(room7),
        Node::Room(room8), Node::Room(room9), Node::Room(room10),Node::Room(room11),
    ]);

    println!("{}{}{}{}{}{}{}{}{}{}{}{}", map.nodes[0], map.nodes[1], map.nodes[2], map.nodes[3], map.nodes[4], map.nodes[5], map.nodes[6], map.nodes[7], map.nodes[8], map.nodes[9], map.nodes[10], map.nodes[11]);
}