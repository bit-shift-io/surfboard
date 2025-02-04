
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Dock {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl Dock {
    pub const ALL: [Dock; 4] = [
        Dock::Top,
        Dock::Bottom,
        Dock::Left,
        Dock::Right,
    ];
}

impl std::fmt::Display for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Dock::Top => "Top",
                Dock::Bottom => "Bottom",
                Dock::Left => "Left",
                Dock::Right => "Right",
            }
        )
    }
}