/// Map algebra from raster files.
///
/// #
///

extern crate gdal;
extern crate ndarray;

#[macro_use]
extern crate approx;

use std::path::Path;
//use std::collections::HashMap;

use gdal::raster::{Dataset, RasterBand};

use ndarray::{Array, Array2};

fn open(filename: &str) -> Dataset {
    let path = Path::new(filename);
    Dataset::open(path).unwrap()
}

fn get_map(filename: &str) -> Array2<f32> {
    let dataset = open(filename);
    let rasterband: RasterBand = dataset.rasterband(1).unwrap();
    let shape = dataset.size();
    let band = rasterband.read_as::<f32>((0, 0), shape, shape).unwrap();

    let (rows, cols) = shape;
    Array::from_shape_vec((cols, rows), band.data).unwrap()
}

pub fn near(data1: Array2<f32>, data2: Array2<f32>, epsilon: f32) -> bool {
    for diff in (data1 - data2).iter() {
        assert_relative_eq!(*diff, 0.0, epsilon = epsilon);
    }
    true
}

pub fn linear_combination(
    weight1: f32,
    data1: Array2<f32>,
    weight2: f32,
    data2: Array2<f32>,
) -> Array2<f32> {
    (weight1 * data1) + (weight2 * data2)
}

fn main() {
    let data1 = get_map("../data/data1.asc");
    println!("{:?}", data1);

    let data2 = get_map("../data/data2.asc");
    println!("{:?}", data2);

    let data3 = get_map("../data/data3.asc");
    println!("{:?}", data3);

    let data4 = get_map("../data/data4.asc");
    println!("{:?}", data4);

    let result = get_map("../data/result.asc");
    println!("{:?}", result);

    let combination: Array2<f32> = Array::from_shape_vec(
        (3, 4),
        vec![
            0.643743,
            0.697576,
            0.748226,
            0.798649,
            0.642703,
            0.690461,
            0.782892,
            -32768.0,
            0.476069,
            0.746179,
            -32768.0,
            -32768.0,
        ],
    ).unwrap();

    assert!(near(combination, result, std::f32::EPSILON));
    //assert!(near(combination, result, 1e-6f32));
}


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_algebra() {
        let weight1: f32 = 0.3;
        let weight2: f32 = 0.7;

        let data1: Array2<f32> = Array::from_shape_vec((2, 3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
            .unwrap();

        let data2: Array2<f32> =
            Array::from_shape_vec((2, 3), vec![2.0, 5.0, 8.0, 9.0, 13.0, 23.0]).unwrap();

        let result: Array2<f32> =
            Array::from_shape_vec((2, 3), vec![1.7, 4.1, 6.5, 7.5, 10.6, 17.9]).unwrap();

        let combination: Array2<f32> = linear_combination(weight1, data1, weight2, data2);

        //assert!(near(combination, result, std::f32::EPSILON));
        assert!(near(combination, result, 1e-6f32));
    }
}
