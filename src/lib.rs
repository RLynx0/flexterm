pub use flexbox::{ContentSpacing, Flexbox, Orientation};
pub use size::{Size, SizeComponent};

mod flexbox;
mod size;

pub trait Flex {
    fn size(&self) -> Size;
    fn render(&self, available_height: usize, available_width: usize) -> String;
    fn render_minimal(&self) -> String {
        self.render(self.size().height.min(), self.size().width.min())
    }
}

impl<S> Flex for S
where
    S: ToString,
{
    fn size(&self) -> Size {
        Size {
            height: size::SizeComponent::Fixed(self.to_string().lines().count()),
            width: size::SizeComponent::Fixed(
                self.to_string()
                    .lines()
                    .map(|l| l.chars().count())
                    .fold(0, usize::max),
            ),
        }
    }

    fn render(&self, _: usize, _: usize) -> String {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Buffer(pub Size);
impl Flex for Buffer {
    fn size(&self) -> Size {
        self.0.to_owned()
    }

    fn render(&self, _: usize, _: usize) -> String {
        String::new()
    }
}

pub struct HorizontalBar(pub char);
impl Flex for HorizontalBar {
    fn size(&self) -> Size {
        Size {
            height: size::SizeComponent::Fixed(1),
            width: size::SizeComponent::Stretch { min: 0 },
        }
    }
    fn render(&self, _: usize, available_width: usize) -> String {
        self.0.to_string().repeat(available_width)
    }
}

pub struct VerticalBar(pub char);
impl Flex for VerticalBar {
    fn size(&self) -> Size {
        Size {
            height: size::SizeComponent::Stretch { min: 0 },
            width: size::SizeComponent::Fixed(1),
        }
    }
    fn render(&self, available_height: usize, _: usize) -> String {
        (0..available_height)
            .map(|_| self.0.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
