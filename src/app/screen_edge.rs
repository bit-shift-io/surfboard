
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenEdge {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl ScreenEdge {
    pub const ALL: [ScreenEdge; 4] = [
        ScreenEdge::Top,
        ScreenEdge::Bottom,
        ScreenEdge::Left,
        ScreenEdge::Right,
    ];
}

impl std::fmt::Display for ScreenEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScreenEdge::Top => "Top",
                ScreenEdge::Bottom => "Bottom",
                ScreenEdge::Left => "Left",
                ScreenEdge::Right => "Right",
            }
        )
    }
}