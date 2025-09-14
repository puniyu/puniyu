use puniyu_core::bot::get_bot;

#[test]
fn test_get_bot() {
    let bot = get_bot(0);
    assert!(bot.is_some());
}

#[test]
fn test_get_bot_with_id() {
    let bot = get_bot("123456");
    assert!(bot.is_some());
}
