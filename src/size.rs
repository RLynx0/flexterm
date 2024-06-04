#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Size {
    pub height: SizeComponent,
    pub width: SizeComponent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeComponent {
    Fixed(usize),
    Stretch { min: usize },
    MinMax { min: usize, max: usize },
}
impl SizeComponent {
    pub fn min(&self) -> usize {
        match self {
            SizeComponent::Fixed(min)
            | SizeComponent::Stretch { min }
            | SizeComponent::MinMax { min, max: _ } => *min,
        }
    }
}
