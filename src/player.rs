use shipyard::Unique;

use crate::shared::Shape;

#[derive(Debug, Unique)]
pub struct Player {
    pub shape: Shape,
}
