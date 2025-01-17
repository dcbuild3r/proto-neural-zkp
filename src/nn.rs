#[cfg(test)]
pub mod test {
    use crate::layers::{
        conv::*, flatten::*, fully_connected::*, maxpool::*, normalize::*, relu::*,
    };
    use ndarray::{Array1, Array2, Array3, Array4, Ix1, Ix3};
    use ndarray_rand::{rand::SeedableRng, rand_distr::Uniform, RandomExt};
    use rand::rngs::StdRng;

    #[test]
    fn neural_net() {
        let seed = 694201337;
        let mut rng = StdRng::seed_from_u64(seed);

        println!(
            "{:<20} | {:<15} | {:<15} | {:<15}",
            "layer", "output shape", "#parameters", "#ops"
        );
        println!("{:-<77}", "");

        // input
        let x = Array3::random_using((120, 80, 3), Uniform::<f32>::new(-10.0, 10.0), &mut rng);

        // conv layer
        // kernel
        let f = Array4::random_using((32, 5, 5, 3), Uniform::<f32>::new(-10., 10.), &mut rng);
        let Conv2D::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = convolution(&x, &f);

        let (dim_x, dim_y, dim_z) = x.dim();

        assert_eq!(x.dim(), (116, 76, 32));

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // max pooling
        // kernel side
        let s = 2;

        let MaxPool::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = max_pooling_layer(&x, s);

        assert_eq!(x.dim(), (58, 38, 32));

        let (dim_x, dim_y, dim_z) = x.dim();

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // relu layer
        let ReLU::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = relu_layer(&x.into_dyn());

        let x = x.into_dimensionality::<Ix3>().unwrap();

        assert_eq!(x.dim(), (58, 38, 32));

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // conv layer

        // kernel
        let f = Array4::random_using((32, 5, 5, 32), Uniform::<f32>::new(-10., 10.), &mut rng);
        let Conv2D::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = convolution(&x.into_dimensionality::<Ix3>().unwrap(), &f);

        let (dim_x, dim_y, dim_z) = x.dim();

        assert_eq!(x.dim(), (54, 34, 32));

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // max pooling
        let MaxPool::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = max_pooling_layer(&x, 2);

        assert_eq!(x.dim(), (27, 17, 32));

        let (dim_x, dim_y, dim_z) = x.dim();

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // relu layer

        let ReLU::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = relu_layer(&x.into_dyn());

        let x = x.into_dimensionality::<Ix3>().unwrap();

        assert_eq!(x.dim(), (27, 17, 32));

        println!(
            "{} |  ({}, {}, {}) | {} |  {}",
            name, dim_x, dim_y, dim_z, n_params, n_multiplications
        );

        // flatten

        let Flatten::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = flatten_layer(&x);

        assert_eq!(x.len(), 14688);

        println!(
            "{} |  ({}x1) | {} |  {}",
            name,
            x.len(),
            n_params,
            n_multiplications
        );

        // fully connected

        let weights =
            Array2::random_using((1000, 14688), Uniform::<f32>::new(-10.0, 10.0), &mut rng);
        let biases = Array1::random_using(1000, Uniform::<f32>::new(-10.0, 10.0), &mut rng);

        let FCLayer::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = fully_connected(&x, &weights, &biases);

        println!(
            "{} |  ({}x1) | {} |  {}",
            name,
            x.len(),
            n_params,
            n_multiplications
        );

        // relu layer

        let ReLU::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = relu_layer(&x.into_dyn());

        let x = x.into_dimensionality::<Ix1>().unwrap();

        assert_eq!(x.len(), 1000);

        println!(
            "{} |  ({}) | {} |  {}",
            name,
            x.len(),
            n_params,
            n_multiplications
        );

        // fully connected

        let weights = Array2::random_using((5, 1000), Uniform::<f32>::new(-10.0, 10.0), &mut rng);
        let biases = Array1::random_using(5, Uniform::<f32>::new(-10.0, 10.0), &mut rng);

        let FCLayer::<f32> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = fully_connected(&x, &weights, &biases);

        println!(
            "{} |  ({}x1) | {} |  {} \n final output: \n{}",
            name,
            x.len(),
            n_params,
            n_multiplications,
            x
        );

        let x = x.mapv(|x| x as i128);

        // normalization
        let Normalize::<f64> {
            output: x,
            n_params,
            n_multiplications,
            name,
        } = normalize(&x);

        println!("final output (normalized):\n{}", x);
    }
}
