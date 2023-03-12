/*
const noiseData = {
    ywrapb: 4,
    ywrap: 16,
    zwrapb: 8,
    size: 4095,
    octaves: 4,
    falloff: 0.55,
    perlin: undefined,
    iter: min(width, height)
}
const scaled_cosine = t => ((6*t - 15)*t + 10)*t*t*t,
noiseDetail = (o, f) => [noiseData.octaves, noiseData.falloff] = [o, f]
const p = noiseData
const noise = (x, y=0, z=0) => {
    //localize globals
    const random = Math.random, cos = Math.cos, PI = Math.PI

    //create variables
    let xi = x | 0, yi = y | 0, zi = z | 0, xf = x - xi, yf = y - yi, zf = z - zi, rxf, ryf, r = 0, ampl = 0.5, n1, n2, n3

    //create noise array
    if (!p.perlin) {
        const size = p.size + 1
        p.perlin = new Array(size)
        for(let i = size; i--;){
            p.perlin[i] = random()
        }
    }
    const perlin = p.perlin, size = p.size

    //fix some problems
    x < 0 && (x = -x)
    y < 0 && (y = -y)
    z < 0 && (z = -z)

    //compute noise
    for (let o = 0, l = p.octaves; o < l; o++) {
        let of = xi + (yi << p.ywrapb) + (zi << p.zwrapb), smoothed = t => ((6 * t - 15) * t + 10)* t * t * t
        rxf = smoothed(xf)
        ryf = smoothed(yf)
        n1 = perlin[of & p.size]
        n1 += rxf * (perlin[(of + 1) & size] - n1)
        n2 = perlin[(of + p.ywrap) & size]
        n2 += rxf * (perlin[(of + p.ywrap + 1) & size] - n2)
        n1 += ryf * (n2 - n1)

        of += p.zwrap
        n2 = p.perlin[of & size]
        n2 += rxf * (perlin[(of + 1) & size] - n2)
        n3 = perlin[(of + p.ywrap) & size]
        n3 += rxf * (perlin[(of + p.ywrap + 1) & size] - n3)
        n2 += ryf * (n3 - n2)

        n1 += 0.5 * (1 - cos(zf * PI)) * (n2 - n1)

        r += n1 * ampl
        ampl *= p.falloff
        xi <<= 1
        xf *= 2
        yi <<= 1
        yf *= 2
        zi <<= 1
        zf *= 2
        xf >= 1.0 && (xi++, xf--)
        yf >= 1.0 && (yi++, yf--)
        zf >= 1.0 && (zi++, zf--)
    }
    //return value
    return r
}

*/

use wasm_bindgen::prelude::*;
use std::f64::consts;
use rand::prelude::*;

pub struct PerlinValues{
    ywrapb: i32,
    ywrap: i32,
    zwrapb: i32,
    size: i32,
    octaves: i32,
    falloff: f64,
    perlin: Vec<f64>,
    iter: f64
}



pub fn smoothed(t:f64)->f64{
    ((6.0*t - 15.0)*t + 10.0)*t*t*t
}

#[wasm_bindgen]
pub fn perlin_noise(x:i32 ,y:i32, z:i32, width:f64, height:f64)->f64{
    let mut rng = rand::thread_rng(); //Creates a random thread rng. use rng.gen() to create random float

    let mut foo = 0.0;
    if width>height{
        foo = height;
    }else{
        foo = width;
    }


    //Noise Data Implementation as P
    let mut p: PerlinValues = PerlinValues{
        ywrapb: 4,
        ywrap: 16,
        zwrapb: 8,
        size: 4095,
        octaves: 4,
        falloff: 0.55,
        perlin: vec![],
        iter: foo,
    };

    //Creating variables
    let mut xi = x.clone();
    let mut yi = y.clone();
    let mut zi = z.clone();
    let mut xf = x - xi;
    let mut yf = y - yi;
    let mut zf = z - zi;
    let mut rxf;
    let mut ryf;
    let mut r = 0.0;
    let mut ampl = 0.5;
    let mut n1;
    let mut n2;
    let mut n3;

    //Creating Noise Vector
    if p.perlin.len() == 0 {
        let size = &p.size + 1;
        for i in 0..(size as i32){
            p.perlin.push(rng.gen())
        }
    }

    let perlin = p.perlin.clone();
    let size = p.size.clone();

    //We need them to be absolute values
    let x = x.abs();
    let y = y.abs();
    let z = z.abs();

    for i in 0.. (p.octaves as i32){
        let mut of = xi + (yi << p.ywrap) + (zi << p.zwrapb);
        rxf = smoothed(xf.into());
        ryf = smoothed(yf.into());
        n1 = perlin[(of & p.size) as usize];
        n1 += rxf * (perlin[((of + 1) & size) as usize] - n1);
        n2 = perlin[((of + p.ywrap) & size) as usize];
        n2 += rxf * (perlin[((of + p.ywrap + 1) & size) as usize] - n2);
        n1 += ryf * (n2 - n1);

        of += p.zwrapb;
        n2 = p.perlin[(of & size) as usize];
        n2 += rxf * (perlin[((of + 1) & size) as usize] - n2);
        n3 = perlin[((of + p.ywrap) & size) as usize];
        n3 += rxf * (perlin[((of + p.ywrap + 1) & size) as usize] - n3);
        n2 += ryf * (n3 - n2);

        n1 += 0.5 * (1 as f64 - (zf as f64 * consts::PI).cos()) * ((n2 - n1) as f64);

        r += n1 * ampl;
        ampl *= p.falloff;
        xi <<= 1;
        xf *= 2;
        yi <<= 1;
        yf *= 2;
        zi <<= 1;
        zf *= 2;
        if xf >= 1 {
            xi += 1;
            xf -= 1;
        }
        if yf >= 1 {
            yi += 1;
            yf -= 1;
        }
        if zf >= 1 {
            zi += 1;
            zf -= 1;
        }
    }

    r
}
