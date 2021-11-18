mod dice;

mod grid;
use grid::Node as Node;

mod starting_area_1;

fn main() {

    let room0 = starting_area_1::new();
    let room1 = starting_area_1::new();
    let room2 = starting_area_1::new();
    let room3 = starting_area_1::new();

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