use crate::{Flex, Size, SizeComponent};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ContentSpacing {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

pub struct Flexbox {
    orientation: Orientation,
    align_content: ContentSpacing,
    justify_content: ContentSpacing,
    content: Vec<Box<dyn Flex>>,
}

impl Flexbox {
    pub fn vertical() -> Self {
        Flexbox {
            orientation: Orientation::Vertical,
            ..Default::default()
        }
    }
    pub fn horizontal() -> Self {
        Flexbox {
            orientation: Orientation::Horizontal,
            ..Default::default()
        }
    }

    pub fn with_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }

    pub fn align_content(&mut self, align: ContentSpacing) -> &mut Self {
        self.align_content = align;
        self
    }

    pub fn justify_content(&mut self, justify: ContentSpacing) -> &mut Self {
        self.justify_content = justify;
        self
    }

    pub fn add_item(&mut self, item: impl Flex + 'static) -> &mut Self {
        self.content.push(Box::new(item));
        self
    }

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

impl Default for Flexbox {
    fn default() -> Self {
        Flexbox {
            orientation: Orientation::default(),
            align_content: ContentSpacing::default(),
            justify_content: ContentSpacing::default(),
            content: Vec::default(),
        }
    }
}

impl Flex for Flexbox {
    fn size(&self) -> Size {
        Size {
            height: self.content.iter().map(|c| c.size().height).fold(
                SizeComponent::Stretch { min: 0 },
                match self.orientation {
                    Orientation::Vertical => add_size,
                    Orientation::Horizontal => max_size,
                },
            ),
            width: self.content.iter().map(|c| c.size().width).fold(
                SizeComponent::Stretch { min: 0 },
                match self.orientation {
                    Orientation::Vertical => max_size,
                    Orientation::Horizontal => add_size,
                },
            ),
        }
    }

    fn render(&self, available_height: usize, available_width: usize) -> String {
        todo!()
    }
}

fn add_size(current: SizeComponent, conetent: SizeComponent) -> SizeComponent {
    SizeComponent::Stretch {
        min: current.min().saturating_add(conetent.min()),
    }
}

fn max_size(current: SizeComponent, conetent: SizeComponent) -> SizeComponent {
    SizeComponent::Stretch {
        min: current.min().max(conetent.min()),
    }
}
