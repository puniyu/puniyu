use regex::Regex;
use std::pin::Pin;

pub trait CommandBuilder {
    fn name(&self) -> &'static str;

    fn regex(&self) -> Regex;

    /// TODO: e占位，未实现
    fn run(&self, e: &'static str) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}
