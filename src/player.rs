use std::fmt;

/*
    Player
    
    This one is going to be complex. Where should I start? Well.. the player needs an icon.
    ^, >, v, <
    The player will obviously eventually be an object. Their icon should be an enum like tile does it.


*/

pub struct Player {
    pub icon: PlayerIcon
}

pub enum PlayerIcon {
    NorthFacing,
    SouthFacing,
    EastFacing,
    WestFacing
}

impl fmt::Display for PlayerIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerIcon::NorthFacing => write!(f, "{} ", "^"),
            PlayerIcon::SouthFacing => write!(f, "{} ", "v"),
            PlayerIcon::EastFacing => write!(f, "{} ", "<"),
            PlayerIcon::WestFacing => write!(f, "{} ", ">"),
        }
    }
}