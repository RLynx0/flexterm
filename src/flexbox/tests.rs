use crate::{ContentSpacing, Flex, Flexbox, Size, SizeComponent};

#[test]
fn empty() {
    let flex = Flexbox::default();
    let expected_size = Size {
        height: SizeComponent::Fixed(0),
        width: SizeComponent::Fixed(0),
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_v() {
    let flex = Flexbox::vertical().add_item("1234").take();
    let expected_size = Size {
        height: SizeComponent::Fixed(1),
        width: SizeComponent::Fixed(4),
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_h() {
    let flex = Flexbox::horizontal().add_item("1234").take();
    let expected_size = Size {
        height: SizeComponent::Fixed(1),
        width: SizeComponent::Fixed(4),
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
        height: SizeComponent::Fixed(3),
        width: SizeComponent::Fixed(6),
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
        height: SizeComponent::Fixed(3),
        width: SizeComponent::Fixed(6),
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
        height: SizeComponent::Fixed(4),
        width: SizeComponent::Fixed(8),
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
        height: SizeComponent::Fixed(1),
        width: SizeComponent::Fixed(4 + 1 + 8 + 6),
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
        height: SizeComponent::Fixed(6),
        width: SizeComponent::Fixed(8),
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
        height: SizeComponent::Fixed(3),
        width: SizeComponent::Fixed(4 + 8 + 6),
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_justify_stretch_v() {
    let flex = Flexbox::vertical()
        .justify_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 0 },
        width: SizeComponent::Fixed(0),
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_justify_stretch_h() {
    let flex = Flexbox::horizontal()
        .justify_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Fixed(0),
        width: SizeComponent::Stretch { min: 0 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_align_stretch_v() {
    let flex = Flexbox::vertical()
        .align_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Fixed(0),
        width: SizeComponent::Stretch { min: 0 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_align_stretch_h() {
    let flex = Flexbox::horizontal()
        .align_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 0 },
        width: SizeComponent::Fixed(0),
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_justify_align_stretch_v() {
    let flex = Flexbox::vertical()
        .justify_content(ContentSpacing::Stretch)
        .align_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 0 },
        width: SizeComponent::Stretch { min: 0 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn emypty_justify_align_stretch_h() {
    let flex = Flexbox::horizontal()
        .justify_content(ContentSpacing::Stretch)
        .align_content(ContentSpacing::Stretch)
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 0 },
        width: SizeComponent::Stretch { min: 0 },
    };
    assert_eq!(flex.size(), expected_size);
}

#[test]
fn single_string_stretch_v() {
    let flex = Flexbox::default()
        .justify_content(ContentSpacing::Stretch)
        .align_content(ContentSpacing::Stretch)
        .add_item("1234")
        .take();
    let expected_size = Size {
        height: SizeComponent::Stretch { min: 1 },
        width: SizeComponent::Stretch { min: 4 },
    };
    assert_eq!(flex.size(), expected_size);
}
