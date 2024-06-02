use crate::{Flex, Size, SizeComponent};

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

    fn base_size(&self) -> Size {
        let flow_size = match self.justify_content {
            ContentSpacing::Stretch => SizeComponent::Stretch { min: 0 },
            _ => SizeComponent::Fixed(0),
        };
        let counter_size = match self.align_content {
            ContentSpacing::Stretch => SizeComponent::Stretch { min: 0 },
            _ => SizeComponent::Fixed(0),
        };

        match self.orientation {
            Orientation::Vertical => Size {
                height: flow_size,
                width: counter_size,
            },
            Orientation::Horizontal => Size {
                height: counter_size,
                width: flow_size,
            },
        }
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
        let base = self.base_size();
        let fold_flow = |a: SizeComponent, b: SizeComponent| a.add(&b);
        let fold_counter = |a: SizeComponent, b: SizeComponent| a.max(&b);

        Size {
            height: self.content.iter().map(|c| c.size().height).fold(
                base.height,
                match self.orientation {
                    Orientation::Vertical => fold_flow,
                    Orientation::Horizontal => fold_counter,
                },
            ),
            width: self.content.iter().map(|c| c.size().width).fold(
                base.width,
                match self.orientation {
                    Orientation::Vertical => fold_counter,
                    Orientation::Horizontal => fold_flow,
                },
            ),
        }
    }

    fn render(&self, available_height: usize, available_width: usize) -> String {
        todo!()
    }
}
