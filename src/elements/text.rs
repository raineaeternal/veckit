#[derive(Default)]
pub struct Text {
    pub s: String,
    pub color: [f32; 4],
    pub rich_text: bool,
}

impl<T> From<T> for Text
where
    T: ToString,
{
    fn from(s: T) -> Text {
        return Text {
            s: s.to_string(),
            ..Default::default()
        };
    }
}
