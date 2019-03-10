#[macro_use] extern crate microprofile;

fn run_test()
{
	microprofile::scope!("foo", "fisk");
	for _ in 0..10
	{
		microprofile::scope!("foo", "bar", 0xff00ff00);
		microprofile::scope!("foo", "fest");
		for _ in 0..10
		{
			microprofile::scope!("foo", "baz");
			std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 /100));
		}
	}
}



fn main() {
		microprofile::init();
		microprofile::set_enable_all_groups(true);
		microprofile::start_auto_flip(20);
		run_test();
		microprofile::stop_auto_flip();
		microprofile::dump_file_immediately("foo.html", "");
		microprofile::shutdown();

}
