
use std::{fs};
use std::time::SystemTime;
use bigdecimal::num_traits::Euclid;
use image::{GenericImageView, ImageReader, Rgba};
use minifb::{Window, WindowOptions};


fn to_binary(n:u32) -> Vec<String> {
    let mut curr_a:Vec<bool> = Vec::new();
    let mut nxt:u32=n;
    let mut str_b = String::new();
    for _ in 0..32{
        let res = nxt.div_rem_euclid(&2);
        curr_a.push(res.1 > 0);

        nxt = res.0;
    }

    curr_a.reverse();
    for val in curr_a {
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
    let width = u32::from_be_bytes([vals[0], vals[1], vals[2], vals[3]]);
    let height = u32::from_be_bytes([vals[4], vals[5], vals[6], vals[7]]);
    vec![height, width]
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
    let mut conv_data = fs::read(path_to_converted).unwrap();
    let mut sig_extracted = conv_data.clone();
    let sig: Vec<u8> = vec![
        102, 114, 111, 103, 105, 110, 97, 108, 111, 103, 32,
        119, 97, 115, 32, 104, 101, 114, 101, 32, 117, 119, 117,
        32, 58, 51
    ];
    sig_extracted.truncate(26);

    println!("Detecting if signatures match");
    if sig_extracted == sig{
        println!("Signatures match!!! :D");
    }else{
        println!("Signatures dont match D:<")
    }
    conv_data.drain(0..26);
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
    let sig: Vec<u8> = vec![
        102, 114, 111, 103, 105, 110, 97, 108, 111, 103, 32,
        119, 97, 115, 32, 104, 101, 114, 101, 32, 117, 119, 117,
        32, 58, 51
    ];
    let sig_len= 26;
    let now = SystemTime::now();
    println!("sig_len: {}", sig_len);
    let output_directory = "output/";
    let extension = ".rbf";
    let mut file_name = "tung";
    let convert = true;
    let convert_from = "SampleImages/img1.png";

    fs::create_dir_all(output_directory).unwrap();
    let binding = output_directory.to_owned() + file_name;
    file_name = &*binding;
    let full_name = file_name.to_owned()+extension;

    if convert {

        let img = ImageReader::open(convert_from).expect("Could not open image").decode().unwrap();
        let mut pixels = Vec::new();
        let mut encoded_pixels = Vec::new();

        let width = to_binary(img.width());
        let height = to_binary(img.height());
        println!("Height: {}, Width: {}", img.height(), img.width());

        // encode image
        for y in 0..img.dimensions().1 {
            for x in 0..img.dimensions().0 {
                let pix = img.get_pixel(x, y);

                pixels.push((pix[0], pix[1], pix[2], pix[3]));
                encoded_pixels.push(pix[0]);
                encoded_pixels.push(pix[1]);
                encoded_pixels.push(pix[2]);
                encoded_pixels.push(pix[3]);
            }
        }

        // prepare 32 bit resolution headers as eight 8 bit values
        let header = [&width[..], &height[..]].concat();
        let header_bytes = header
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>();

        let bytes = encoded_pixels;


        // combine the headers and bytes

        let comb = [&sig[..], &header_bytes[..], &bytes[..]].concat();
        println!("Time to encode: {}",now.elapsed().unwrap().as_millis());
        let now2 = SystemTime::now();
        fs::write(full_name.clone(), comb).expect("Should be able to write");

        comp(&*full_name.clone(), convert_from);
        println!("Time to write: {}",now2.elapsed().unwrap().as_millis());
    }


    // reading
    let now3 = SystemTime::now();
    let mut data = fs::read(full_name.clone()).unwrap();
    data.drain(0..sig_len);
    let mut header_bits = data.clone();
    header_bits.truncate(8);
    let extracted_headers = headers_parser(header_bits);
    println!("{:?}", extracted_headers);

    let w = extracted_headers[1];
    let h = extracted_headers[0];


    let mut win = Window::new(
        "RGBA",
        w as usize,
        h as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).expect("Window creation failed");

    let src_w = w as usize;
    let src_h = h as usize;
    let mut buf = vec![0u32; src_w * src_h];
    let rgba = data.clone().drain(8..data.len()).collect::<Vec<_>>();

    // turn into buffer
    for i in 0..(h*w) {

        let r = rgba[(i * 4) as usize] as u32;
        let g = rgba[(i * 4 + 1)as usize] as u32;
        let b = rgba[(i * 4 + 2) as usize] as u32;

        buf[i as usize] = (r << 16) | (g << 8) | b;
    }
    println!("Time to convert to buffer: {}", now3.elapsed().unwrap().as_millis());

    let mut frame_buf: Vec<u32> = Vec::new();
    while win.is_open() {
        let (win_w, win_h) = win.get_size();
        if win_w == 0 || win_h == 0 {
            win.update();
            continue;
        }

        if frame_buf.len() != win_w * win_h {
            frame_buf.resize(win_w * win_h, 0);
        }
        frame_buf.fill(0);

        let src_aspect = src_w as f32 / src_h as f32;
        let win_aspect = win_w as f32 / win_h as f32;
        let (draw_w, draw_h) = if win_aspect > src_aspect {
            (((win_h as f32) * src_aspect).round() as usize, win_h)
        } else {
            (win_w, ((win_w as f32) / src_aspect).round() as usize)
        };

        let draw_w = draw_w.max(1).min(win_w);
        let draw_h = draw_h.max(1).min(win_h);
        let off_x = (win_w - draw_w) / 2;
        let off_y = (win_h - draw_h) / 2;

        for y in 0..draw_h {
            let src_y = y * src_h / draw_h;
            let dst_row = (off_y + y) * win_w + off_x;
            let src_row = src_y * src_w;
            for x in 0..draw_w {
                let src_x = x * src_w / draw_w;
                frame_buf[dst_row + x] = buf[src_row + src_x];
            }
        }

        win.update_with_buffer(&frame_buf, win_w, win_h).unwrap();
    }

}
