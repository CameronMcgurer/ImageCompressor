use csc411_image::{self, RgbImage, Rgb, Read, Write};
//use array2::Array2;

/// Returns a vector of data representing the input image with a trimmed width and height
///
/// # Arguments:
/// * 'orig_img`: the input image (.ppm)
pub fn trim_image(orig_img: RgbImage) -> Vec<Rgb> {
    // sets new width and height to be even
    let trim_w = orig_img.width & !1_u32;
    let trim_h = orig_img.height & !1_u32;
    let mut new_data: Vec<Rgb> = vec![];
    // only trim the data if the width or height is odd
    if orig_img.width % 2 == 1 || orig_img.height % 2 == 1 {
        for r in 0..trim_h {
            for c in 0..trim_w {
                let index = (r * orig_img.width + c) as usize;
                new_data.push(orig_img.pixels[index].clone());
            }
        }
    }
    else {
        new_data = orig_img.pixels.clone();
    }
    new_data
}

/// Returns a vector containing the float values of the rgb values from the input vector
///
/// # Arguments:
/// * 'int_rgb`: A vector that holds the rgb vlaues of the image
/// * 'denom': The denominattor of the input image
pub fn rgb_to_float(int_rgb: Vec<Rgb>, denom: u16) -> Vec<(f64, f64, f64)> {
    let float_rgb: Vec<(f64, f64, f64)> = int_rgb.into_iter().map(|pixel|(
        pixel.red as f64 / denom as f64, 
        pixel.green as f64 / denom as f64, 
        pixel.blue as f64 / denom as f64)).collect();
    float_rgb
}

/// Returns a vector containing rgb values
///
/// # Arguments:
/// * 'float_rgb`: A vector of tuples which each have float values
pub fn float_to_rgb(float_rgb: Vec<(f64, f64, f64)>) -> Vec<Rgb> {
    let int_rgb: Vec<Rgb> = float_rgb.into_iter().map(|(r,g,b)| Rgb {
        red: (r * 255.0) as u16, 
        green: (g * 255.0) as  u16, 
        blue: (b * 255.0) as u16}).collect();
    int_rgb
}

/// Returns a vector containing video component values
///
/// # Arguments:
/// * 'float_rgb`: A vector that holds the float rgb vlaues of the image
pub fn float_to_vid(float_rgb: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let float_vid: Vec<(f64, f64, f64)> = float_rgb.into_iter().map(|(r,g,b)|(
        0.299 * r + 0.587 * g + 0.114 * b, 
        -0.168736 * r - 0.331264 * g + 0.5 * b, 
        0.5 * r - 0.418688 * g - 0.081312 * b)).collect();
    float_vid
}

/// Returns a vector containing the float rgb values
///
/// # Arguments:
/// * 'float_vid`: A vector containing video component values of each pixel
/// * 'denom': The denominattor of the input image
pub fn vid_to_float(float_vid: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    let float_rgb: Vec<(f64, f64, f64)> = float_vid.into_iter().map(|(y,pb,pr)|(
        1.0 * y + 0.0 * pb + 1.402 * pr, 
        1.0 * y - 0.344136 * pb - 0.714136 * pr,
        1.0 * y + 1.772 * pb + 0.0 * pr)).collect();
    float_rgb
}

/// Returns an RgbImage from the 'filename'
///
/// # Arguments:
/// * 'filename`: A filename (should be a .ppm file)
pub fn read_image(filename: &str) -> RgbImage {
    let orig_img = RgbImage::read(Some(filename)).unwrap();
    orig_img
}

/// Writes an image to standard out
///
/// # Arguments:
/// * 'vec`: A vector that holds the rgb vlaues of the image
/// * 'new_w`: The width of the decompressed image
/// * 'new_h`: The height of the decompressed image
pub fn write_image(vec: Vec<Rgb>, new_w: u32, new_h: u32) {
    let decomp_img = RgbImage {
        pixels: vec,
        width: new_w,
        height: new_h,
        denominator: 255,
    };
    decomp_img.write(None).unwrap();
}