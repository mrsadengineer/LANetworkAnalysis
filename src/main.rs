mod checknetwork;
mod ippinger;
mod ippinger3ips;
mod output_slicing;

fn main() {
    println!("Hello, LANetwork Analysis App!!");
    //checknetwork::checknetwork();
    //ippinger::pingrange();
    //ippinger3ips::pingrangeall();
    output_slicing::slice_up_output();
}
