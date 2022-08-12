use std::fmt::Write;
use noise::{utils::*, Fbm, NoiseFn, OpenSimplex, Seedable, Perlin, MultiFractal};

byond_fn!(fn simplex(x, y, seed, freq, octaves, lacunarity){
    _simplex(x, y, seed, freq, octaves, lacunarity).ok()
});

pub fn _simplex(_x:&str, _y:&str, _seed:&str, _freq:&str, _octaves:&str, _lacunarity:&str) -> Result<String, bool> {
    let size_x: i32 = _x.parse().unwrap();
    let size_y: i32 = _y.parse().unwrap();
    let seed: u32 = _seed.parse().unwrap();
    let freq: f64 = _freq.parse().unwrap();
    let octaves: usize = _octaves.parse().unwrap();
    let lacunarity: f64 = _lacunarity.parse().unwrap();
    // let gain: f32 = _gain.parse().unwrap();
    let mut fbm = Fbm::<>::new();
    fbm = fbm.set_seed(seed);
    fbm = fbm.set_frequency(freq);
    fbm = fbm.set_octaves(octaves);
    fbm = fbm.set_lacunarity(lacunarity);
    let mut string = String::new();
    for y in 0..size_y{
        for x in 0..size_x{
            let val_at_cords = fbm.get([x as f64, y as f64]);
            let _ = write!(string, "{},", val_at_cords.to_string());
        }
    }
    Ok(string)
}