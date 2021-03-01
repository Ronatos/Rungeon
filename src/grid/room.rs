struct Room {
    grid: TileGrid,
    exits: Exits,
    longest_dimension: u8,
    kind: RoomKind,
}

struct RoomExit {
    location: ExitLocation,
    leads_to: RoomKind
}

enum Exits {
    One([RoomExit; 1]),
    Two([RoomExit; 2]),
    Three([RoomExit; 3]),
    Four([RoomExit; 4]),
    Five([RoomExit; 5]),
    Six([RoomExit; 6])
}

enum ExitLocation {
    Top,
    Bottom,
    Left,
    Right
}

enum RoomKind {
    Chamber,
    Entrance,
    Passage
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}