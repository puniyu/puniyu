use async_trait::async_trait;
use puniyu_types::task::{Task, TaskBuilder, TaskId};
use std::sync::{Arc, Mutex};

#[test]
fn test_task_id_from_u64() {
	let task_id: TaskId = 42u64.into();
	match task_id {
		TaskId::Index(idx) => assert_eq!(idx, 42),
		_ => panic!("Expected TaskId::Index"),
	}
}

#[test]
fn test_task_id_from_string() {
	let task_id: TaskId = "test_task".to_string().into();
	match task_id {
		TaskId::Name(name) => assert_eq!(name, "test_task"),
		_ => panic!("Expected TaskId::Name"),
	}
}

#[test]
fn test_task_id_from_str() {
	let task_id: TaskId = "test_task".into();
	match task_id {
		TaskId::Name(name) => assert_eq!(name, "test_task"),
		_ => panic!("Expected TaskId::Name"),
	}
}

#[test]
fn test_task_creation() {
	let task = Task { name: "my_task", cron: "0 0 * * * *" };

	assert_eq!(task.name, "my_task");
	assert_eq!(task.cron, "0 0 * * * *");
}

struct TestTask {
	name: &'static str,
	cron: &'static str,
	run_count: Arc<Mutex<u32>>,
}

impl TestTask {
	fn new(name: &'static str, cron: &'static str) -> Self {
		Self { name, cron, run_count: Arc::new(Mutex::new(0)) }
	}

	fn get_run_count(&self) -> u32 {
		*self.run_count.lock().unwrap()
	}
}

#[async_trait]
impl TaskBuilder for TestTask {
	fn name(&self) -> &'static str {
		self.name
	}

	fn cron(&self) -> &'static str {
		self.cron
	}

	async fn run(&self) {
		let mut count = self.run_count.lock().unwrap();
		*count += 1;
	}
}

#[tokio::test]
async fn test_task_name() {
	let task = TestTask::new("test_task", "0 0 * * * *");
	assert_eq!(task.name(), "test_task");
}

#[tokio::test]
async fn test_task_cron() {
	let task = TestTask::new("test_task", "0 */5 * * * *");
	assert_eq!(task.cron(), "0 */5 * * * *");
}

#[tokio::test]
async fn test_task_run() {
	let task = TestTask::new("test_task", "0 0 * * * *");

	assert_eq!(task.get_run_count(), 0);

	task.run().await;
	assert_eq!(task.get_run_count(), 1);

	task.run().await;
	assert_eq!(task.get_run_count(), 2);
}

#[tokio::test]
async fn test_task_multiple_tasks() {
	let task1 = TestTask::new("task1", "0 0 * * * *");
	let task2 = TestTask::new("task2", "0 */10 * * * *");

	assert_eq!(task1.name(), "task1");
	assert_eq!(task2.name(), "task2");

	task1.run().await;
	task2.run().await;
	task2.run().await;

	assert_eq!(task1.get_run_count(), 1);
	assert_eq!(task2.get_run_count(), 2);
}

#[test]
fn test_cron_expressions() {
	let task1 = Task { name: "hourly", cron: "0 0 * * * *" };
	assert_eq!(task1.cron, "0 0 * * * *");

	let task2 = Task { name: "daily", cron: "0 0 0 * * *" };
	assert_eq!(task2.cron, "0 0 0 * * *");

	let task3 = Task { name: "every_5_min", cron: "0 */5 * * * *" };
	assert_eq!(task3.cron, "0 */5 * * * *");
}
