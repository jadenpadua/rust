use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
	let stdout: std::io::Stdout = stdout();
	let message: String = String::from("Hello fellow Rustaceans!");
	let width: usize = message.chars().count();
	
	let mut writer: BufWriter<std::io::StdoutLock> = BufWriter::new(stdout.lock());
	say(&message, width, &mut writer).unwrap();	
}
