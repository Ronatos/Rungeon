mod dice;

mod grid;
use grid::Node as Node;

mod room;

fn main() {

    let room0 = room::starting_area_9::new();
    let room1 = room::starting_area_9::new();
    let room2 = room::starting_area_9::new();
    let room3 = room::starting_area_9::new();
    let room4 = room::starting_area_9::new();
    let room5 = room::starting_area_9::new();
    let room6 = room::starting_area_9::new();
    let room7 = room::starting_area_9::new();
    let room8 = room::starting_area_9::new();
    let room9 = room::starting_area_9::new();
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