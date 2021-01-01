use std::process::Command;

pub fn pingrange() {
    let mut ipstr: String = "192.168.0.".to_owned();
    let ipad0: &str = "0";
    let ipad1: &str = "1";

    ipstr.push_str(ipad1);
    println!("{}", ipstr);

    // println!(ipad1String);


    let mut cmd = Command::new("ping");
    cmd.arg(ipstr);

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
