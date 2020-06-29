extern crate rusty_machine;
extern crate rand;

use rusty_machine::linalg::{Matrix, BaseMatrix};

use rand::thread_rng;
use rand::distributions::Distribution;
use rand_distr::Normal;
use std::io;
use std::vec::Vec;
use std::fs::read_to_string;
use serde::Deserialize;

/*
const CENTROIDS: [f64;6] = [
    22.5, 40.5,
    38.0, 50.0,
    25.5, 48.0
];
*/

// const NOISE: f64 = 1.8;
// const SAMPLES_PER_CENTROID: usize = 2000;

#[derive(Deserialize)]
struct Config {
    centroids: [f64;6],
    noise: f64,
    samples_per_centroid: usize,
}

fn generate_data(centroids: &Matrix<f64>, 
    points_per_centroid: usize, noise: f64) -> Matrix<f64>
{
    assert!(centroids.cols() > 0, "centroids cannot be empty");
    assert!(centroids.rows() > 0, "centroids cannot be empty");
    assert!(noise >= 0f64, "centroids cannot be empty");

    let mut raw_cluster_data = 
        Vec::with_capacity(centroids.rows() *
        points_per_centroid * centroids.cols());
    
    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..points_per_centroid {
        for centroid in centroids.iter_rows() {
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroids.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            raw_cluster_data.extend(point);
        }
    }

    Matrix::new(centroids.rows() * points_per_centroid,
                centroids.cols(), raw_cluster_data)
}

fn main() -> Result<(), std::io::Error> {

    let toml_config_str = read_to_string(
        "config/generate.toml"
    )?;

    let config: Config = toml::from_str(&toml_config_str)?;

    let centroids = Matrix::new(3, 2, config.centroids.to_vec());
    let samples = generate_data(&centroids, config.samples_per_centroid, config.noise);
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"]);
    for sample in samples.iter_rows() {
        writer.serialize(sample)?;
    }
    Ok(())
}