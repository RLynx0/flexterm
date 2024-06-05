pub use flexbox::{ContentAlign, Flexbox, Flow};
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

/// A Flex container that always renders it's content as small as possible.
/// Useful for constraining a `Flexbox`
///
/// # Examples
/// ```
/// use flexterm::{Flex, Flexbox, Minimal};
///
/// let cont = "some_string";
/// let cont_size = cont.size();
///
/// // `Flexbox` will always be of size `Stretch`,
/// // even if its content has a fixed size.
/// let fbox = Flexbox::default()
///     .add_item(cont)
///     .take();
/// let fbox_size = fbox.size();
///
/// // `Minimal` will always be of size `Fixed`,
/// // even if its content has a variable size.
/// let min = Minimal(fbox);
/// let min_size = min.size();
///
/// assert_ne!(fbox_size, cont_size);
/// assert_ne!(fbox_size, min_size);
/// assert_eq!(min_size, cont_size);
/// ```
pub struct Minimal<T>(pub T);
impl<T> Flex for Minimal<T>
where
    T: Flex,
{
    fn size(&self) -> Size {
        let size = self.0.size();
        Size {
            height: SizeComponent::Fixed(size.height.min()),
            width: SizeComponent::Fixed(size.width.min()),
        }
    }
    fn render(&self, _: usize, _: usize) -> String {
        self.0.render_minimal()
    }
}
