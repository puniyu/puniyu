mod template;

pub struct RenderInfo {
    /// 渲染器索引
    index: usize,
    /// 渲染器名称
    name: String,
}
pub trait RenderBase {
    fn info() -> RenderInfo;
}
