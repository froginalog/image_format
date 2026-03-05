
use std::{fs, u32};

use bigdecimal::num_traits::Euclid;
use image::{GenericImageView, ImageReader, Rgba};
use minifb::{Window, WindowOptions};

fn encode(n:u8) -> String{



    let mut curra:Vec<bool> = Vec::new();
    let mut nxt:u8=n;
    let mut str_b = String::new();
    for _ in 0..8{
        let res = nxt.div_rem_euclid(&2);
        curra.push(res.1 > 0);

        nxt = res.0;
    }

    curra.reverse();
    for val in curra{
        if val{
            str_b.push('1');
        }else{
            str_b.push('0');
        }

    }

    str_b
}


fn to_binary(n:u32) -> Vec<String> {
    let mut curra:Vec<bool> = Vec::new();
    let mut nxt:u32=n;
    let mut str_b = String::new();
    for _ in 0..32{
        let res = nxt.div_rem_euclid(&2);
        curra.push(res.1 > 0);

        nxt = res.0;
    }

    curra.reverse();
    for val in curra{
        if val{
            str_b.push('1');
        }else{
            str_b.push('0');
        }

    }
    println!("{}",str_b.clone());
    let chars: Vec<char> = str_b.chars().collect();
    let split = chars.chunks(8)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    split

}

fn headers_parser(vals:Vec<u8>) -> Vec<u32> {
    if vals.len() != 8{
        println!("Not 8 bytes");
        std::process::exit(1);
    }
    //println!("{:#?}",vals);
    let width = [vals[0],vals[1],vals[2],vals[3]];
    let height = [vals[4],vals[5],vals[6],vals[7]];
    let mut height_str = String::new();
    let mut width_str = String::new();
    for val in height{
        height_str = height_str + &*encode(val)
    }
    for val in width{
        width_str = width_str + &*encode(val)
    }
    // println!("{:#?}",height_str);
    // println!("{:#?}",width_str);
    let mut final_vals = Vec::new();

    final_vals.push(u32::from_str_radix(&height_str, 2).unwrap());
    final_vals.push(u32::from_str_radix(&width_str, 2).unwrap());

    final_vals
}

fn pixel_compare(c_pixel:Vec<u8>,o_pixel:Rgba<u8>) -> bool{
    let o_pixel_vec = vec![o_pixel[0], o_pixel[1], o_pixel[2], o_pixel[3]];
    println!("{:?}", c_pixel);
    println!("{:?}", o_pixel_vec);
    if c_pixel == o_pixel_vec{
        true
    }else{
        false
    }

}
fn comp(path_to_converted:&str, path_to_original:&str){
    println!("comp ran");
    let og_image = ImageReader::open(path_to_original).expect("Could not decode original path").decode().unwrap();
    let conv_data = fs::read(path_to_converted).unwrap();

    let mut header_bits = conv_data.clone();
    header_bits.truncate(8);
    let headers = headers_parser(header_bits);
    let w:u32 = headers[1];
    let h:u32 = headers[0];
    let og_w:u32 = og_image.width().try_into().unwrap();
    let og_h:u32 = og_image.height().try_into().unwrap();
    let rgba_data = conv_data.clone().drain(8..conv_data.len()).collect::<Vec<_>>();
    let rgba_chunks = rgba_data.chunks(4);
    println!("Comparing dimensions");
    if w == og_w{
        println!("Widths are equal");
    }else {
        println!("Widths are not equal, og width: {}, new width: {}", og_w,w);
    }
    if h == og_h{
        println!("Heights are equal");
    }else {
        println!("Heights are not equal, og height: {}, new height: {}", og_h,h);
    }

    println!("Comparing top left pixel");
    if pixel_compare(rgba_chunks.clone().nth(0).unwrap().to_vec(),og_image.get_pixel(0,0)){
        println!("top left pixels are equal");
    }else{
        println!("top left pixels are not equal");
    }
    println!("Comparing bottom left pixel");
    if pixel_compare(rgba_chunks.clone().nth((w*(h - 1)) as usize).unwrap().to_vec(), og_image.get_pixel(0, og_image.height()-1)){
        println!("bottom left pixels are equal");
    }
    println!("Comparing top right pixel");
    if pixel_compare(rgba_chunks.clone().nth((w - 1) as usize).unwrap().to_vec(), og_image.get_pixel(og_image.width()-1, 0)){
        println!("top right pixels are equal");
    }
    println!("Comparing bottom right pixel");
    if pixel_compare(rgba_chunks.clone().nth((w*h) as usize -1).unwrap().to_vec(),og_image.get_pixel(og_image.width()-1, og_image.height()-1)){
        println!("bottom right pixels are equal");
    }
    println!("Headers: {:?}", headers);
}

fn main() {

    let extension = ".gunk";
    let file_name = "tung";
    let full_name = file_name.to_owned() +extension;

    let img = ImageReader::open("equality_test.png").expect("REASON").decode().unwrap();
    let mut pixels = Vec::new();
    let mut fstr = Vec::new();

    let width = to_binary(img.width());
    let height = to_binary(img.height());
    let mut size_headers = Vec::new();
    size_headers.push(&width);
    size_headers.push(&height);
    println!("Height: {}, Width: {}", img.height(), img.width());

    // encode image
    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let pix = img.get_pixel(x,y);

            pixels.push((pix[0], pix[1],pix[2],pix[3]));
            let r = encode(pix[0]);
            let g = encode(pix[1]);
            let b = encode(pix[2]);
            let a = encode(pix[3]);
            fstr.push(r);
            fstr.push(g);
            fstr.push(b);
            fstr.push(a);

        }

    }

    // prepare 32 bit resolution headers as eight 8 bit values
    let header =  [&width[..],&height[..]].concat();
    let header_bytes = header
        .iter()
        .map(|s| u8::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>();
    let bytes: Vec<u8> = fstr
        .iter()
        .map(|s| u8::from_str_radix(s, 2).unwrap())
        .collect();

    // combine the headers and bytes
    let comb = [&header_bytes[..],&bytes[..]].concat();
    fs::write(full_name.clone(), comb).expect("Should be able to write");

    comp(&*full_name.clone(), "equality_test.png");



    // reading
    let data = fs::read(full_name.clone()).unwrap();
    let mut header_bits = data.clone();
    header_bits.truncate(8);
    let extracted_headers = headers_parser(header_bits);
    println!("{:?}", extracted_headers);

    let w = extracted_headers[1];
    let h = extracted_headers[0];


    let mut win = Window::new("RGBA", w as usize,h as usize, WindowOptions::default()).expect("Window creation failed");

    let mut buf = vec![0u32; (w*h) as usize];
    let rgba = data.clone().drain(8..data.len()).collect::<Vec<_>>();

    // turn into buffer
    for i in 0..(h*w) {

        let r = rgba[(i * 4) as usize] as u32;
        let g = rgba[(i * 4 + 1)as usize] as u32;
        let b = rgba[(i * 4 + 2) as usize] as u32;

        buf[i as usize] = (r << 16) | (g << 8) | b;
    }


    while win.is_open() {
        win.update_with_buffer(&buf, w as usize, h as usize).unwrap();
    }

}
