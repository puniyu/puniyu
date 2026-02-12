use std::sync::Arc;

use puniyu_loader::Loader;
use puniyu_plugin::Plugin;

struct TestLoader {
    name: &'static str,
}

impl Loader for TestLoader {
    fn name(&self) -> &'static str {
        self.name
    }

    fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
        vec![]
    }
}

#[test]
fn test_loader_creation() {
    let loader = TestLoader { name: "test_loader" };
    assert_eq!(loader.name(), "test_loader");
}

#[test]
fn test_loader_name() {
    let loader = TestLoader { name: "my_loader" };
    assert_eq!(loader.name(), "my_loader");
}

#[test]
fn test_loader_plugins_empty() {
    let loader = TestLoader { name: "test_loader" };
    let plugins = loader.plugins();
    assert_eq!(plugins.len(), 0);
}

#[test]
fn test_loader_equality() {
    let loader1 = TestLoader { name: "loader1" };
    let loader2 = TestLoader { name: "loader1" };
    let loader3 = TestLoader { name: "loader2" };

    assert_eq!(loader1.name(), loader2.name());
    assert_ne!(loader1.name(), loader3.name());
}

#[test]
fn test_loader_trait_object() {
    let loader: Box<dyn Loader> = Box::new(TestLoader { name: "boxed_loader" });
    assert_eq!(loader.name(), "boxed_loader");
}

#[test]
fn test_loader_different_names() {
    let loader1 = TestLoader { name: "loader_a" };
    let loader2 = TestLoader { name: "loader_b" };
    let loader3 = TestLoader { name: "loader_c" };

    assert_eq!(loader1.name(), "loader_a");
    assert_eq!(loader2.name(), "loader_b");
    assert_eq!(loader3.name(), "loader_c");
}

#[test]
fn test_loader_special_characters() {
    let loader = TestLoader { name: "loader_123" };
    assert_eq!(loader.name(), "loader_123");
}

#[test]
fn test_loader_unicode_name() {
    let loader = TestLoader { name: "加载器" };
    assert_eq!(loader.name(), "加载器");
}
