mod dice;

mod grid;
use grid::Node as Node;

mod room;

fn main() {

    let room0 = room::starting_area_5::new();
    let room1 = room::starting_area_5::new();
    let room2 = room::starting_area_5::new();
    let room3 = room::starting_area_5::new();

    let map = grid::Grid::new(2, vec![
        // Row 1
        Node::Room(room0),
        Node::Room(room1),

        // Row 2
        Node::Room(room2),
        Node::Room(room3)
    ]);

    println!("{}{}{}{}", map.nodes[0], map.nodes[1], map.nodes[2], map.nodes[3]);
}