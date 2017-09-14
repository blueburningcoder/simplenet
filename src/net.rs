extern crate nalgebra;
extern crate rand;

use self::rand::*;
use self::nalgebra::*;

use mnist::*;


#[derive(Debug)]
pub struct Network {
    num_layers: usize,
    sizes: Vec<i32>,
    biases: Vec<DMatrix<f32>>,
    weights: Vec<DMatrix<f32>>,
}


impl Network {

    /// creating a new supposed neural network.
    /// The sizes vec!([2, 3, 1]) would generate a new
    /// network with 2 input-Neurons, one hidden layer with 3,
    /// and one output-neuron.
    pub fn new(sizes: Vec<i32>) -> Network {

        let layers = sizes.len();
        Network {
            num_layers: layers,
            sizes: sizes.clone(),

            biases:
                sizes.clone()
                    .iter()
                    .skip(1)
                    .map(|s| DMatrix::new_random(*s as usize, 1))
                    .collect(),

            weights:
                sizes.iter()
                    .skip(1)
                    .zip(sizes.iter())
                    .map(|(y, x)| DMatrix::new_random(*y as usize,
                                                      *x as usize))
                    .collect(),
            }
    }


    /// returns the output of the network for the input vector 'a'.
    pub fn feedforward(&self, a: Vec<f32>) -> Vec<f32> {
        let mut a = a;
        for (b, w) in self.biases.iter().zip(self.weights.iter()) {
            let s = w * DVector::from_iterator(a.len(), a) + b;
            a = sigmoid(s.iter());
        }
        a
    }


    /// Train the network using mini-batch stochastic gradient descent.
    /// The "training_data" is a list of tuples "(x, y)" representing the
    /// training inputs and the desired outputs. The other parameters are
    /// self-explanatory.
    pub fn stochastic_gradient_descent(&mut self, training_data: Vec<Picture>,
        epochs: i32, mini_batch_size: usize, eta: f32) {
        let mut training_data = training_data;
        let n = training_data.len();
        for _ in 0..epochs {
            training_data = randomize(training_data);
            for k in 0..(n / mini_batch_size) {
                let triter = training_data.iter().skip(k * mini_batch_size);
                let mini_batch = triter.take(mini_batch_size);
                self.update_mini_batch(mini_batch, eta);
            }
        }
    }


    // stub
    fn update_mini_batch<'a, I: Iterator<Item = &'a Picture>>(&mut self,
        batch: I, eta: f32) {
        // TODO: write
    }
}

fn randomize<T: Clone>(mut vec: Vec<T>) -> Vec<T> {
    let slice = vec.as_mut_slice();
    rand::thread_rng().shuffle(slice);
    slice.to_vec()
}


/// simple sigmoid function over vectors of floats.
// fn sigmoid<'a, F: 'a + Float, T : Iterator<Item = &'a F>>(i: T) -> Vec<F>
fn sigmoid<'a, T : Iterator<Item = &'a f32>>(i: T) -> Vec<f32>
{
    i
//        .map(|z| 1. as F / (1. as F + ((-(*z)).exp() as F)) )
        .map(|z| 1. / (1. + (-z).exp() ) )
        .collect()
}


#[cfg(test)]
mod test {

    #[test]
    fn simpletenettest() {
        let n = super::Network::new(vec![2, 3, 1]);
        assert_eq!(n.sizes, vec![2, 3, 1]);
        assert_eq!(n.biases.iter().count(), 3);
        assert_eq!(n.biases[0].shape(), (2, 1));
        assert_eq!(n.weights.iter().count(), 2);
        assert_eq!(n.weights[0].shape(), (3, 2));
    }

}
