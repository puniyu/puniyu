pub use puniyu_bot::Bot;
pub use puniyu_bot::BotId;

pub fn get_bot<'b>(bot_id: impl Into<BotId<'b>>) -> Option<Bot> {
    use puniyu_bot::BotRegistry;
    let bot_id = bot_id.into();
    match bot_id {
        BotId::Index(id) => BotRegistry::get_with_index(id),
        BotId::SelfId(name) => BotRegistry::get_with_bot_id(name),
    }
}

pub fn get_all_bot() -> Vec<Bot> {
    use puniyu_bot::BotRegistry;
    BotRegistry::all()
}
pub fn get_bot_count() -> usize {
    get_all_bot().len()
}
