#[derive(Clone, Debug)]
pub struct Size {
    pub height: SizeComponent,
    pub width: SizeComponent,
}

#[derive(Clone, Debug)]
pub enum SizeComponent {
    Fixed(usize),
    Stretch { min: usize },
    MinMax { min: usize, max: usize },
}

impl SizeComponent {
    pub fn min(&self) -> usize {
        match self {
            SizeComponent::Fixed(n) => *n,
            SizeComponent::Stretch { min } => *min,
            SizeComponent::MinMax { min, max: _ } => *min,
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        use SizeComponent::*;
        match (self, other) {
            (Fixed(a), Fixed(b)) => Fixed(*a.max(b)),
            (Stretch { min: a }, Stretch { min: b }) => Stretch { min: *a.max(b) },
            (
                MinMax {
                    min: min_a,
                    max: max_a,
                },
                MinMax {
                    min: min_b,
                    max: max_b,
                },
            ) => MinMax {
                min: *min_a.max(min_b),
                max: *max_a.max(max_b),
            },

            (Stretch { min }, Fixed(c)) => Stretch { min: *c.max(min) },
            (Fixed(c), Stretch { min }) => Stretch { min: *c.max(min) },
            (Stretch { min: a }, MinMax { min: b, max: _ }) => Stretch { min: *a.max(b) },
            (MinMax { min: a, max: _ }, Stretch { min: b }) => Stretch { min: *a.max(b) },
            (Fixed(c), MinMax { min, max }) => MinMax {
                min: *c.max(min),
                max: *c.max(max),
            },
            (MinMax { min, max }, Fixed(c)) => MinMax {
                min: *c.max(min),
                max: *c.max(max),
            },
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        use SizeComponent::*;
        match (self, other) {
            (Fixed(a), Fixed(b)) => Fixed(a + b),
            (Stretch { min: a }, Stretch { min: b }) => Stretch { min: a + b },
            (
                MinMax {
                    min: min_a,
                    max: max_a,
                },
                MinMax {
                    min: min_b,
                    max: max_b,
                },
            ) => MinMax {
                min: min_a + min_b,
                max: max_a + max_b,
            },

            (Stretch { min }, Fixed(c)) => Stretch { min: c + min },
            (Fixed(c), Stretch { min }) => Stretch { min: c + min },
            (Stretch { min: a }, MinMax { min: b, max: _ }) => Stretch { min: a + b },
            (MinMax { min: a, max: _ }, Stretch { min: b }) => Stretch { min: a + b },
            (Fixed(c), MinMax { min, max }) => MinMax {
                min: c + min,
                max: c + max,
            },
            (MinMax { min, max }, Fixed(c)) => MinMax {
                min: c + min,
                max: c + max,
            },
        }
    }
}
