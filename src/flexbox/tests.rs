use crate::{Flex, Flexbox, Size, SizeComponent};

#[test]
fn empty() {
    let flex = Flexbox::default();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 0 },
        width: SizeComponent::Stretch { min: 0 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_v() {
    let flex = Flexbox::vertical().add_item("1234").take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 1 },
        width: SizeComponent::Stretch { min: 4 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_h() {
    let flex = Flexbox::horizontal().add_item("1234").take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 1 },
        width: SizeComponent::Stretch { min: 4 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_v_multiline() {
    let flex = Flexbox::vertical()
        .add_item(
            "1234\n\
            123456\n\
            12",
        )
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 3 },
        width: SizeComponent::Stretch { min: 6 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_h_multiline() {
    let flex = Flexbox::horizontal()
        .add_item(
            "1234\n\
            123456\n\
            12",
        )
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 3 },
        width: SizeComponent::Stretch { min: 6 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn strings_v() {
    let flex = Flexbox::vertical()
        .add_item("1234")
        .add_item("1")
        .add_item("12345678")
        .add_item("123456")
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 4 },
        width: SizeComponent::Stretch { min: 8 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn strings_h() {
    let flex = Flexbox::horizontal()
        .add_item("1234")
        .add_item("1")
        .add_item("12345678")
        .add_item("123456")
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 1 },
        width: SizeComponent::Stretch { min: 4 + 1 + 8 + 6 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn strings_multiline_v() {
    let flex = Flexbox::vertical()
        .add_item(
            "1234\n\
            1",
        )
        .add_item(
            "12345678\n\
            12345\n\
            1",
        )
        .add_item("123456")
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 6 },
        width: SizeComponent::Stretch { min: 8 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn strings_multiline_h() {
    let flex = Flexbox::horizontal()
        .add_item(
            "1234\n\
            1",
        )
        .add_item(
            "12345678\n\
            12345\n\
            1",
        )
        .add_item("123456")
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 3 },
        width: SizeComponent::Stretch { min: 4 + 8 + 6 },
    };
    assert_eq!(flex.size(), expected_size);
}
