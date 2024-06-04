use flexterm::{Buffer, Flex, Flexbox, HorizontalBar, Size, SizeComponent, VerticalBar};

fn main() {
    let cont = Flexbox::vertical()
        .add_item(HorizontalBar('═'))
        .add_item("Wat ze fuck")
        .add_item(HorizontalBar('═'))
        .add_item(titlecard("nuh-uh!"))
        .add_item(HorizontalBar('═'))
        .add_item(titlecard("What ze fuck!!!"))
        .add_item(HorizontalBar('═'))
        .add_item(titlecard(
            "No you...\n\
            No you fucking don't!",
        ))
        .add_item(HorizontalBar('═'))
        .take();

    println!("{:#?}", cont.size());
    // println!("{}", cont.render_minimal());
}

fn titlecard(input: &str) -> Flexbox {
    let content = Flexbox::horizontal()
        .add_item(Buffer(Size {
            height: SizeComponent::Stretch { min: 0 },
            width: SizeComponent::Fixed(1),
        }))
        .add_item(input.to_string())
        .add_item(Buffer(Size {
            height: SizeComponent::Stretch { min: 0 },
            width: SizeComponent::Fixed(1),
        }))
        .add_item(VerticalBar('│'))
        .take();

    let last_line = Flexbox::horizontal()
        .add_item(HorizontalBar('─'))
        .add_item('╯')
        .take();

    Flexbox::vertical()
        .add_item(content)
        .add_item(last_line)
        .take()
}
