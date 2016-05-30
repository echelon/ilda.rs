// Copyright (c) 2016 Brandon Thomas <bt@brand.io>
extern crate ilda;
extern crate image;

use ilda::data::IldaEntry;
use ilda::limit;
use ilda::parser::read_file;
use std::fs::File;
use std::path::Path;

const WIDTH : u32 = 1200;
const HEIGHT : u32 = 1200;

pub fn main() {
  //let result = read_file("./examples/ildatest.ild")
  let result = read_file("./examples/nyancat.ild")
  //let result = read_file("./examples/files/+burst.ild")
  //let result = read_file("./examples/files/100prcnt.ild")
    .ok().unwrap();

  //println!("{:?}", result);

  let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);

  for entry in result {
    //println!("Entry: {:?}", entry);
    match entry {
      IldaEntry::HeaderEntry(header) => {
        println!("Header: {:?}", header);
      },
      IldaEntry::TcPoint2dEntry(pt) => {
        if pt.r < 100 && pt.g < 100 && pt.b < 100 {
          continue;
        }

        //println!("{}, {}, {}", pt.r, pt.g, pt.b);

        //let pixel = image::Rgb([255u8, pt.g, pt.b]);
        let pixel = image::Rgb([pt.r, pt.g, pt.b]);
        //let pixel = image::Rgb([255u8, 255u8, 255u8]);
        let x = t_x(pt.x, WIDTH);
        let y = t_y(pt.y, HEIGHT);
        imgbuf.put_pixel(x, y, pixel);
      },
      IldaEntry::IdxPoint3dEntry(pt) => {
        let pixel = image::Rgb([255u8, 255u8, 255u8]);
        let x = t_x(pt.x, WIDTH);
        let y = t_y(pt.y, HEIGHT);
        imgbuf.put_pixel(x, y, pixel);
      },
      _ => {},
    }
  }

  /*for i in (0..800) {
    let perc = (i as f32) / (800 as f32);
    let cf = 255.0 * perc;
    let c = cf as u8;

    let pixel = image::Rgb([c, c, c]);
    imgbuf.put_pixel(i, i, pixel);
  }*/

  let ref mut fout = File::create(&Path::new("output.png"))
    .unwrap();
  let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

/// Transform x-coordinate.
fn t_x(x : i16, img_width: u32) -> u32 {
  // FIXME: This is abhorrent.
  let ix = (x as i32).saturating_add(limit::MAX_X as i32);
  let scale = (img_width as f64) / (limit::WIDTH as f64);
  ((ix as f64 * scale) as i32).abs() as u32
}

/// Transform y-coordinate.
fn t_y(y : i16, img_height: u32) -> u32 {
  // FIXME: This is abhorrent.
  // NB: Have to invert y since the vertical coordinate system transforms.
  let iy = ((y * -1) as i32).saturating_add(limit::MAX_Y as i32);
  let scale = (img_height as f64) / (limit::HEIGHT as f64);
  ((iy as f64 * scale) as i32).abs() as u32
}

