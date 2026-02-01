use std::process::Command;

fn main() {
    let output = Command::new("ls")
		.arg("-ls")
		.output()
		.expect("Failed to execute command");

	let stdout = String::from_utf8_lossy(&output.stdout);

	println!("{}", stdout);
	println!("{}", output.status.success());
}
