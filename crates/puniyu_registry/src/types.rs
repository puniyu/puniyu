#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    Plugin(u64),
    Adapter(u64),
}