use std::process::Command;

fn callPingCommand(ipstrnp: String) {
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

    callPingCommand(ipstr00);
    callPingCommand(ipstr01);
    callPingCommand(ipstr02);
    callPingCommand(ipstr03);
    callPingCommand(ipstr04);
    callPingCommand(ipstr05);
    callPingCommand(ipstr06);
    callPingCommand(ipstr07);
    callPingCommand(ipstr08);
    callPingCommand(ipstr09);
    callPingCommand(ipstr10);
    callPingCommand(ipstr11);
    callPingCommand(ipstr12);
    callPingCommand(ipstr13);
    callPingCommand(ipstr14);
    callPingCommand(ipstr15);
    callPingCommand(ipstr16);

}
