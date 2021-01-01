use std::process::Command;

pub fn checknetwork() {
    let mut cmd = Command::new("ipconfig");
    println!("test");
    match cmd.output() {
        Ok(o) => {
            println!("hello");

            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
        }

        Err(e) => {
            println!("there is something wrong {}", e);
        }
    }
}