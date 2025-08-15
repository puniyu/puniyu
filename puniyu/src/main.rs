use puniyu_core::system::system_info::get_memory_info;
fn main() {
    let sys = get_memory_info();
    println!("{:?}", sys);
}
