use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(value: String) -> SubscriberName {
        let is_empty_or_whitespace = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '%'];
        let contains_forbidden_chars = value
            .chars()
            .any(|char| forbidden_characters.contains(&char));

        if is_empty_or_whitespace || is_too_long | contains_forbidden_chars {
            panic!("{} is not a valid subscriber name.", value);
        }

        Self(value)
    }
}
