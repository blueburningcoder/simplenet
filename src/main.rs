extern crate simplenetlib;

use simplenetlib::net::*;


fn main() {
    // test();
    // print!("Hello World!");
    /*
    let trainL = read_labels("train-labels-idx1-ubyte")
                    .expect("failed to read file");
    let trainN = read_numbers("train-images-idx3-ubyte")
                    .expect("failed to read file");
    */


    /*
    let testL = read_labels("t10k-labels-idx1-ubyte")
                    .expect("failed to read file");
    let testN = read_numbers("t10k-images-idx3-ubyte")
                    .expect("failed to read file");
    */

    // let combo = mnist::get_pictures("t10k-images-idx3-ubyte",
    //     "t10k-labels-idx1-ubyte").unwrap();

    let net = Network::new(vec![2, 3, 1]);

    println!("{:#?}", net.feedforward(vec![0., 0.]));
}
