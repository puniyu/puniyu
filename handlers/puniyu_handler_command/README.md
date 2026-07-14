# puniyu_handler_command

`puniyu` 的默认命令 Handler，负责响应模式、命令解析、权限、冷却和候选命令执行。

```rust,ignore
use puniyu_handler_command::CommandHandler;

let app = puniyu::App::builder().handler(CommandHandler::new()).build();
```

Handler 按注册顺序执行。命令未匹配或候选要求继续传播时调用后续 Handler；
已处理命令及命令执行错误会在当前层终止。

`Master` 身份来源尚未实现，因此 `ReactiveMode::Master` 和 `Permission::Master` 当前不可用。
