use crate::{Flex, Size, SizeComponent};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub enum Flow {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ContentAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ContentSpacing {
    #[default]
    Between,
    Around,
    Even,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentJustify {
    Align(ContentAlign),
    Space(ContentSpacing),
}
impl ContentJustify {
    pub fn start() -> Self {
        ContentJustify::Align(ContentAlign::Start)
    }
    pub fn center() -> Self {
        ContentJustify::Align(ContentAlign::Center)
    }
    pub fn end() -> Self {
        ContentJustify::Align(ContentAlign::End)
    }
    pub fn space_between() -> Self {
        ContentJustify::Space(ContentSpacing::Between)
    }
    pub fn space_around() -> Self {
        ContentJustify::Space(ContentSpacing::Around)
    }
    pub fn space_even() -> Self {
        ContentJustify::Space(ContentSpacing::Even)
    }
}
impl Default for ContentJustify {
    fn default() -> Self {
        ContentJustify::Align(Default::default())
    }
}

/// A container for an arbitrary amount of elements that implement Flex.
///
/// # Flex-Size
/// Flexbox will always take up as much space as it is given.
/// If it has more space than it's items need, they are rendered depending
/// on the fields `justify_content` and `align_content`.
///
/// `justify_content` refers to how elements are layed out parallel
/// to the Flexbox's orientation.
/// - `Align(Start)` : at the top for vertical flow, at the left for horizontal flow
/// - `Align(End)` : at the bottom for vertical flow, at the right for horizontal flow
/// - `Align(Center)` : in the middle of the flexbox
/// - `Space(Between)` : Distribute leftover space inbetween elements
/// - `Space(Around)` : Distribute leftover space to the left and right of each element
/// - `Space(Even)` : Distribute leftover space inbetween and around elements
///
/// `align_content` refers to how elements are layed out perpendicular
/// to the Flexbox's orientation.
/// - `Start` : at the left for vertical flow, at the top for horizontal flow
/// - `End` : at the right for vertical flow, at the bottom for horizontal flow
/// - `Center` : in the middle of the flexbox
///
/// For more information see `ContentSpacing`
pub struct Flexbox {
    flow: Flow,
    align_content: ContentAlign,
    justify_content: ContentJustify,
    content: Vec<Box<dyn Flex>>,
}

impl Flexbox {
    pub fn vertical() -> Self {
        Flexbox {
            flow: Flow::Vertical,
            ..Default::default()
        }
    }
    pub fn horizontal() -> Self {
        Flexbox {
            flow: Flow::Horizontal,
            ..Default::default()
        }
    }

    pub fn flow(&mut self, flow: Flow) -> &mut Self {
        self.flow = flow;
        self
    }

    pub fn align_content(&mut self, align: ContentAlign) -> &mut Self {
        self.align_content = align;
        self
    }

    pub fn justify_content(&mut self, justify: ContentJustify) -> &mut Self {
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
            flow: Flow::default(),
            align_content: ContentAlign::default(),
            justify_content: ContentJustify::default(),
            content: Vec::default(),
        }
    }
}

impl Flex for Flexbox {
    fn size(&self) -> Size {
        Size {
            height: self.content.iter().map(|c| c.size().height).fold(
                SizeComponent::Stretch { min: 0 },
                match self.flow {
                    Flow::Vertical => add_size,
                    Flow::Horizontal => max_size,
                },
            ),
            width: self.content.iter().map(|c| c.size().width).fold(
                SizeComponent::Stretch { min: 0 },
                match self.flow {
                    Flow::Vertical => max_size,
                    Flow::Horizontal => add_size,
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
