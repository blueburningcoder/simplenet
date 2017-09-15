extern crate nalgebra;
extern crate rand;

use self::rand::*;
use self::nalgebra::*;

use mnist::*;

use std::ops::Mul;

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
    /// and one output-neuron. Initially the weights/biases are random.
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
    /// The "training_data" is a list of Pictures representing the
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


    /// update the network's weights and biases by applying gradient descent
    /// using backpropagation to a single mini batch. The mini_batch is a list
    /// of Pictures and eta is the learning rate.
    fn update_mini_batch<'a, I: Iterator<Item = &'a Picture>>(&mut self,
        batch: I, eta: f32) {
        let mut nabla_b = self.biases.iter()
            .map(|b| DMatrix::zeros(b.shape().0, b.shape().1)).collect();
        let mut nabla_w = self.weights.iter()
            .map(|w| DMatrix::zeros(w.shape().0, w.shape().1)).collect();
        let l = batch.cloned().count() as f32;

        for (delta_nabla_b, delta_nabla_w) in
            batch.map(|b| self.backprop(*b)) { // important!!
            nabla_b = nabla_b.iter().map(|b| b + delta_nabla_b).collect();
            nabla_w = nabla_w.iter().map(|w| w + delta_nabla_w).collect();
        }

        self.biases = self.biases
                            .iter()
                            .zip(nabla_b)
                            .map(|(b, nb)| b - (eta/l) * nb)
                            .collect();
        self.weights = self.weights
                            .iter()
                            .zip(nabla_w)
                            .map(|(w, nw)| w - (eta/l) * nw)
                            .collect();
        // TODO: write
    }


    /// Return a tuple (nabla_b, nabla_w) representingthe gradient for the cost
    /// function C_x. nabla_b and nabla_w are layer-by-layer lists of vecs,
    /// similiar to self.bias and self.weights.
    fn backprop(&mut self, pic: Picture) -> (DVector<f32>, DVector<f32>) {
        // TODO: write
        let mut nabla_b = self.biases.iter()
            .map(|b| DMatrix::zeros(b.shape().0, b.shape().1)).collect();
        let mut nabla_w = self.weights.iter()
            .map(|w| DMatrix::zeros(w.shape().0, w.shape().1)).collect();
        let mut activation: Vec<f32> =
            pic.get_tuple().0.iter().map(|e| *e as f32).collect();
        let mut activations = vec![activation];
        let mut zs = Vec::new();

        for (b, w) in self.biases.iter().zip(self.weights) {
            let z = w.mul(DVector::from_iterator(activation.len(),
                                                 activation.iter())) + b;
            zs.push(z.copy());
            activation = sigmoid(z.iter());
            activations.push(activation);
        }
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
