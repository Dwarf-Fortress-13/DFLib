use colors_transform::Color;
use std::cmp::{max, min};
use image::{Rgba, RgbaImage};


byond_fn!(fn smooth_floor(center, target, cname, tname){
    _smooth_floor(center, target, cname, tname).ok()
});

fn _smooth_floor(center:&str, target:&str, cname:&str, tname:&str) -> Result<String, bool>{
    let center_data:Vec<&str> = center.split(",").collect();
    let target_data:Vec<&str> = target.split(",").collect();
    let mut img = RgbaImage::new(32,32);
    for i in 0..32{
        let cpixel = colors_transform::Rgb::from_hex_str(center_data[i]).unwrap();
        let tpixel = colors_transform::Rgb::from_hex_str(target_data[i]).unwrap();
        let cr:u8 = cpixel.get_red() as u8;
        let cg:u8 = cpixel.get_green() as u8;
        let cb:u8 = cpixel.get_blue() as u8;
        let tr:u8 = tpixel.get_red() as u8;
        let tg:u8 = tpixel.get_green() as u8;
        let tb:u8 = tpixel.get_blue() as u8;
        let rstep = (max(cr,tr)-min(cr,tr))/3;
        let gstep = (max(cg,tg)-min(cg,tg))/3;
        let bstep = (max(cb,tb)-min(cb,tb))/3;
        img.put_pixel(i as u32, 0, Rgba([if(cr>tr){cr-rstep}else{cr+rstep}, if(cg>tg){cg-gstep}else{cg+gstep}, if(cb>tb){cb-bstep}else{cb+bstep}, 255]));
    }
    img.save(format!("tmp/{}{}.png", cname, tname));
    Ok(String::from("1"))
}