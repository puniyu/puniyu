# puniyu_cooldown

统一的冷却管理类型，覆盖全局、机器人、好友、群组与群成员场景。

## 特性

- 🎯 **统一范围**: 使用 `CooldownScope` 描述冷却范围
- 🔌 **统一接口**: 通过 `CooldownRegistry` 管理冷却状态
- ⏱️ **灵活时长**: 支持任意 `Duration` 冷却时间
- 🧹 **自动清理**: 支持清理过期冷却记录

## 示例

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };

if !CooldownRegistry::is_cooling_down(&scope) {
    CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
}

CooldownRegistry::clear_cooldown(&scope).unwrap();
CooldownRegistry::cleanup_expired();
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
