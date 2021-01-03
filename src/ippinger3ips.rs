use std::process::Command;

fn call_ping_command(ipstrnp: String) {
    println!("{}", ipstrnp);
    let mut cmd = Command::new("ping");
    cmd.arg(ipstrnp);

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

pub fn pingrangeall() {
    //let mut ipstr: String = "192.168.0.".to_owned();
    let ipstr00: String = String::from("192.168.0.0");
    let ipstr01: String = String::from("192.168.0.1");
    let ipstr02: String = String::from("192.168.0.2");
    let ipstr03: String = String::from("192.168.0.3");
    let ipstr04: String = String::from("192.168.0.4");
    let ipstr05: String = String::from("192.168.0.5");
    let ipstr06: String = String::from("192.168.0.6");
    let ipstr07: String = String::from("192.168.0.7");
    let ipstr08: String = String::from("192.168.0.8");
    //let mut ipstr: String = "192.168.0.".to_owned();
    let ipstr09: String = String::from("192.168.0.9");
    let ipstr10: String = String::from("192.168.0.10");
    let ipstr11: String = String::from("192.168.0.11");
    let ipstr12: String = String::from("192.168.0.12");
    let ipstr13: String = String::from("192.168.0.13");
    let ipstr14: String = String::from("192.168.0.14");
    let ipstr15: String = String::from("192.168.0.15");
    let ipstr16: String = String::from("192.168.0.16");
    //let ipad0: &str = "11";

    // let ipad1: &str = "1";

    //ipstr.push_str(ipad0);

    // println!(ipad1String);

    call_ping_command(ipstr00);
    call_ping_command(ipstr01);
    call_ping_command(ipstr02);
    call_ping_command(ipstr03);
    call_ping_command(ipstr04);
    call_ping_command(ipstr05);
    call_ping_command(ipstr06);
    call_ping_command(ipstr07);
    call_ping_command(ipstr08);
    call_ping_command(ipstr09);
    call_ping_command(ipstr10);
    call_ping_command(ipstr11);
    call_ping_command(ipstr12);
    call_ping_command(ipstr13);
    call_ping_command(ipstr14);
    call_ping_command(ipstr15);
    call_ping_command(ipstr16);

}
