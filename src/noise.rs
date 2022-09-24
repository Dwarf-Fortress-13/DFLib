use std::fmt::Write;
use noise::{utils::*, Fbm, NoiseFn, OpenSimplex, Seedable, Perlin, MultiFractal};

byond_fn!(fn fbm(x, y, seed, freq, octaves, lacunarity, persistence){
    _fbm(x, y, seed, freq, octaves, lacunarity, persistence).ok()
});

pub fn _fbm(_x:&str, _y:&str, _seed:&str, _freq:&str, _octaves:&str, _lacunarity:&str, _persistence:&str) -> Result<String, bool> {
    let size_x: i32 = _x.parse().unwrap();
    let size_y: i32 = _y.parse().unwrap();
    let seed: u32 = _seed.parse().unwrap();
    let freq: f64 = _freq.parse().unwrap();
    let octaves: usize = _octaves.parse().unwrap();
    let lacunarity: f64 = _lacunarity.parse().unwrap();
    let persistence: f64 = _persistence.parse().unwrap();
    let mut fbm = Fbm::<>::new();
    fbm = fbm.set_seed(seed);
    fbm = fbm.set_frequency(freq);
    fbm = fbm.set_octaves(octaves);
    fbm = fbm.set_lacunarity(lacunarity);
    fbm = fbm.set_persistence(persistence);
    let mut string = String::new();
    for y in 0..size_y{
        for x in 0..size_x{
            let val_at_cords = fbm.get([x as f64, y as f64]);
            let _ = write!(string, "{},", val_at_cords.to_string());
        }
    }
    Ok(string)
}