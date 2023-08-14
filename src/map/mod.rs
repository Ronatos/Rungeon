pub mod room;

use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;

enum RoomGenerated {
    True(Grid),
    False
}

pub fn new(num_rows: usize, num_columns: usize, starting_area_index: usize) -> Grid {
    let min_index: usize = 0;
    let max_index: usize = num_rows * num_columns;

    if num_rows == 0 || num_columns == 0 {
        panic!("Map cannot be generated with {} rows and {} columns.", num_rows, num_columns);
    }
    if starting_area_index > max_index { // usize cannot be < 0, so check is not needed
        panic!("Map cannot be generated. starting_area_index {} is out of bounds.", starting_area_index);
    }

    let mut checklist: Vec<RoomGenerated> = Vec::new();
    for _i in min_index..max_index {
        checklist.push(RoomGenerated::False);
    }
    /* 
        checklist = [
            false, false, false,
            false, false, false,
            false, false, false,
        ]
    */

    // Every map needs a starting area, so we can safely generate this first.
    let starting_area_configuration = dice::roll(10);
    let starting_area: Grid = match starting_area_configuration {
        1 => room::starting_area_1::new(),
        2 => room::starting_area_2::new(),
        3 => room::starting_area_3::new(),
        4 => room::starting_area_4::new(),
        5 => room::starting_area_5::new(),
        6 => room::starting_area_6::new(),
        7 => room::starting_area_7::new(),
        8 => room::starting_area_8::new(),
        9 => room::starting_area_9::new(),
        10 => room::starting_area_10::new(),
        _ => room::starting_area_1::new()
    };

    checklist[starting_area_index] = RoomGenerated::True(starting_area.clone());

    /* 
        With the starting area done, our checklist could look like this
        checklist = [
            false, false, false,
            false, true(something), false,
            false, false, false,
        ]
    */

    // This is where the meat of generating the map really happens.
    let mut rooms_left_to_generate: usize = max_index - 1;
    let mut room_index = starting_area_index;
    let mut room_indexes = vec![room_index]; // Keep track of the room_indexes of rooms we need to handle exits for as we go
    while rooms_left_to_generate > 0 {

        // If room at room_index has a North exit and ((room_index - num_columns) >= min_index) and ((room_index - num_columns) is not already in room_indexes) and (checklist[room_index - num_columns] = RoomGenerated::False), we can build one there.
        //     Build the North room
        //         We will need to pass the room builder 2 parameters: we need it to connect to the South, and the passage width.
        //     Clone the room to the checklist at the room_index
        //         checklist[room_index] = RoomGenerated::True(room_type.clone());
        //     Add the North room's room_index to room_indexes, because we now need to do that one too
        //         room_indexes.push(room_index - num_columns);
        // If room at room_index has a South exit and ((room_index + num_columns) <= max_index) and ((room_index + num_columns) is not already in room_indexes) and (checklist[room_index + num_columns] = RoomGenerated::False), we can build one there.
        //     Build the South room
        //         We will need to pass the room builder 2 parameters: we need it to connect to the North, and the passage width.
        //     Clone the room to the checklist at the room_index
        //         checklist[room_index] = RoomGenerated::True(room_type.clone());
        //     Add the South room's room_index to room_indexes, because we now need to do that one too
        //         room_indexes.push(room_index - num_columns);
        // If room at room_index has a East exit and ((room_index + 1) <= max_index) and ((room_index + 1) is not already in room_indexes) and (checklist[room_index + 1] = RoomGenerated::False) and !(room_index + 1 == 0 || (room_index + 1) % num_columns == 0), we can build one there.
        //     Build the East room
        //         We will need to pass the room builder 2 parameters: we need it to connect to the West, and the passage width.
        //     Clone the room to the checklist at the room_index
        //         checklist[room_index] = RoomGenerated::True(room_type.clone());
        //     Add the East room's room_index to room_indexes, because we now need to do that one too
        //         room_indexes.push(room_index - num_columns);
        // If room at room_index has a West exit and ((room_index - 1) >= min_index) and ((room_index - 1) is not already in room_indexes) and (checklist[room_index - 1] = RoomGenerated::False) and !(room_index % num_columns == 0 || room_index == min_index), we can build one there.
        //     Build the West room
        //         We will need to pass the room builder 2 parameters: we need it to connect to the East, and the passage width.
        //     Clone the room to the checklist at the room_index
        //         checklist[room_index] = RoomGenerated::True(room_type.clone());
        //     Add the West room's room_index to room_indexes, because we now need to do that one too
        //         room_indexes.push(room_index - num_columns);
        // Room at room_index is complete
        //     Remove room_index from room_indexes
        //     Set room_index = room_indexes[0] to prepare to do another room
        //     Decrement rooms_left_to_generate
    }

    // The checklist was convenient, but now we need to load in the proper values to a vector in the correct order.
    let mut map_vector: Vec<Node> = Vec::new();
    for i in &checklist {
        if let RoomGenerated::True(grid) = i {
            map_vector.push(Node::Room(grid.clone()));
        };
    }
    
    let map: Grid = Grid::new(1, map_vector);
    return map;
}