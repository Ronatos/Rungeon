mod grid;

use grid::room as room;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();
    let entrance_index: usize = rng.gen_range(0..=9);
    let entrance_location: room::ExitLocation = rng.gen();

    let mut hash_map: HashMap<usize, Option<grid::Module>> = HashMap::new();
    hash_map.insert(0, None);
    hash_map.insert(1, None);
    hash_map.insert(2, None);
    hash_map.insert(3, None);
    hash_map.insert(4, None);
    hash_map.insert(5, None);
    hash_map.insert(6, None);
    hash_map.insert(7, None);
    hash_map.insert(8, None);

    generate_map_recursively(hash_map, entrance_location, &entrance_index, room::RoomKind::StartingArea, 0, room::RoomKind::Chamber);

    let mut map_modules: Vec<grid::Module> = Vec::new();
    for (index, room) in &hash_map {
        map_modules[*index] = match *room {
            Some(grid) => grid,
            None => panic!("This indicates a bug in generate_map_recursively().")
        };
    }
}

fn generate_map_recursively(hash_map: HashMap<usize, Option<grid::Module>>, entrance_location: room::ExitLocation, room_index: &usize, room_kind: room::RoomKind, previous_room_longest_dimension: u8, previous_room_kind: room::RoomKind) -> HashMap<usize, Option<room::Room>> {
    // if room index is out of bounds or the spot is already full, return
    if room_index < &0 || room_index > &8 || if let Some(Room) = hash_map.get(room_index) { true } else { false } {
        return hash_map
    }

    // the only problem with this is that passages need width information
    // about the room they're branching from.
    // otherwise they could end up outrageously large.
    // a problem to deal with later i think.
    // these functions don't even exist yet
    let r = match room_kind {
        room::RoomKind::Chamber => generate_chamber(entrance_location),
        room::RoomKind::Passage => generate_passage(entrance_location, previous_room_longest_dimension, previous_room_kind),
        room::RoomKind::StartingArea => generate_starting_area(entrance_location)
    };

    // insert the room we created at the location we need it to go
    hash_map.insert(room_index, r);

    // now we need to go through all this room's exits to build out what it's supposed to look like
    for exit in r.exits {
        match exit.location {
            // if the current room has an exit leading up, the next room has to have an exit leading to the bottom
            // this has an impact on both the entrance_location and the room_index of the next room
            // i'm curious if when i upgrade the map size if i'll need to pass in some value like num_columns here
            // and change the math to make this part more generic
            room::ExitLocation::Top => generate_map_recursively(hash_map, room::ExitLocation::Bottom, room_index - 3, exit.leads_to, r.longest_dimension, r.kind),
            room::ExitLocation::Bottom => generate_map_recursively(hash_map, room::ExitLocation::Top, room_index + 3, exit.leads_to, r.longest_dimension, r.kind),
            room::ExitLocation::Left => {
                if room_index % 3 != 0 {
                    generate_map_recursively(hash_map, room::ExitLocation::Right, room_index - 1, exit.leads_to, r.longest_dimension, r.kind);
                }
                else {
                    return hash_map
                }
            },
            room::ExitLocation::Right => {
                if (room_index - 2) % 3 != 0 {
                    generate_map_recursively(hash_map, room::ExitLocation::Left, room_index + 1, exit.leads_to, r.longest_dimension, r.kind);
                }
                else {
                    return hash_map
                }
            }
        }
    }
}
