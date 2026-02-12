#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub bg_color: String,
    pub font_color: String,
    pub usage: i32,
    pub tag_type: String,
}
