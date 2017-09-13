extern crate simplenetlib;

use simplenetlib::*;


fn main() {
    test();
    print!("Hello World!");
    /*
    let trainL = read_labels("train-labels-idx1-ubyte.gz")
                    .expect("failed to read file");
    let trainN = read_numbers("train-images-idx3-ubyte.gz")
                    .expect("failed to read file");
    */


    // let testL = read_labels("t10k-labels-idx1-ubyte.gz")
    //                 .expect("failed to read file");
    let testN = read_numbers("t10k-images-idx3-ubyte.gz")
                    .expect("failed to read file");
}
