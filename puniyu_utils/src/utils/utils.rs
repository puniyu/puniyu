use crate::error::Config as Error;

/// 通用函数，删除嵌套节点
///
/// 该函数提供了一种通用的方法来删除嵌套数据结构中的节点。它通过提供获取和删除操作的回调函数，
/// 可以适用于不同的数据结构类型（如JSON、YAML等）。
///
/// # 参数
///
/// * `value` - 需要操作的数据结构的可变引用
/// * `node_keys` - 表示节点路径的字符串切片数组，例如 ["parent", "child", "grandchild"]
/// * `get_mut_mapping` - 闭包函数，用于获取指定键的可变引用，如果键不存在则返回None
/// * `remove_from_mapping` - 闭包函数，用于从数据结构中删除指定键
///
/// # 返回值
///
/// 返回Result<(), Box<dyn Error>>，成功时返回Ok(())，失败时返回错误信息
///
/// # 泛型参数
///
/// * `V` - 数据结构的类型
/// * `F` - 获取可变引用的闭包类型，需要实现Fn(&mut V, &str) -> Option<&mut V>
/// * `G` - 删除节点的闭包类型，需要实现Fn(&mut V, &str)
///
pub(crate) fn delete_nested_node<V, G, M>(
	value: &mut V,
	node_keys: &[&str],
	get_mut_mapping: G,
	remove_from_mapping: M,
) -> Result<(), Error>
where
	G: for<'a> Fn(&'a mut V, &str) -> Option<&'a mut V>,
	M: Fn(&mut V, &str),
{
	if node_keys.is_empty() {
		return Ok(());
	}

	let current_key = node_keys[0];

	if node_keys.len() == 1 {
		remove_from_mapping(value, current_key);
		Ok(())
	} else if let Some(child_value) = get_mut_mapping(value, current_key) {
		delete_nested_node(child_value, &node_keys[1..], get_mut_mapping, remove_from_mapping)
	} else {
		Ok(())
	}
}
