use std::pin::Pin;

pub trait CommandBuilder: Send + Sync + 'static {
	/// 命令名称
	fn name(&self) -> &'static str;
	/// 命令
	///
	/// ## 示例
	///
	/// ```ignore
	/// echo <String>
	/// ```
	///
	/// ## 说明
	///
	/// - `<String>` 表示参数，参数类型为 `String`
	/// - `echo` 表示命令名称
	fn command(&self) -> &'static str;

	/// 优先级
	fn rank(&self) -> usize;

	/// TODO: e占位，未实现
	fn run(&self, e: &'static str) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}
