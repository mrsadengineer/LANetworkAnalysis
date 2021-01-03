use std::process::Command;


#[allow(dead_code)]
pub fn slice_up_output() {
    let ipstr01: String = String::from("192.168.0.1");
    call_ping_and_parse_results(ipstr01);
}

fn call_ping_and_parse_results(ipstrnp: String) {
    
    
    
    println!("{}", ipstrnp);
    let mut cmd = Command::new("ping");
    cmd.arg(ipstrnp);

    match cmd.output() {
        Ok(o) => {
            println!("hello");

            unsafe {
               // println!("Output: {}", String::from_utf8_unchecked(o.stdout));

               let varis = String::from_utf8_unchecked(o.stdout);

               let mut somethinghhh = varis.lines();
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
                println!("{:?}", somethinghhh.next());
            }
        }

        Err(e) => {
            println!("there is something wrong {}", e);
        }
    }
}
