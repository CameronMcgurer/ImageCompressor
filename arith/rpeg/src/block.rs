use csc411_arith::{chroma_of_index, index_of_chroma};

/// Returns a vector tuples that contain the values (a, b, c, d, avg_pb, avg_pr)
/// which are gotten using a 2x2 block from the input vector
///
/// # Arguments:
/// * 'float_vid`: A vector of tuples that contain the brightness, pb, pr
/// * `trim_w`: The width of the trimmed image
/// * 'trim_h': The height of the trimmed image
pub fn get_block(float_vid: Vec<(f64, f64, f64)>, trim_w: u32, trim_h: u32) 
-> Vec<(f64, f64, f64, f64, f64, f64)> {
    let mut new_pixels: Vec<(f64, f64, f64, f64, f64, f64)> = vec![];

    for row in (0..trim_h).step_by(2) {
        for col in (0..trim_w).step_by(2) {
            // get the y values from each origional pixel
            let y1 = float_vid[(row * trim_w + col) as usize].0;
            let y2 = float_vid[(row * trim_w + col + 1) as usize].0;
            let y3 = float_vid[((row + 1) * trim_w + col) as usize].0;
            let y4 = float_vid[((row + 1) * trim_w + col + 1) as usize].0;
            // get the pb values from each origional pixel
            let pb1 = float_vid[(row * trim_w + col) as usize].1;
            let pb2 = float_vid[(row * trim_w + col + 1) as usize].1;
            let pb3 = float_vid[((row + 1) * trim_w + col) as usize].1;
            let pb4 = float_vid[((row + 1) * trim_w + col + 1) as usize].1;
            // get the pr values from each origional pixel
            let pr1 = float_vid[(row * trim_w + col) as usize].2;
            let pr2 = float_vid[(row * trim_w + col + 1) as usize].2;
            let pr3 = float_vid[((row + 1) * trim_w + col)as usize].2;
            let pr4 = float_vid[((row + 1) * trim_w + col + 1) as usize].2;
            // get a, b, c, d, avg_pb, avg_pr 
            let a = (y4 + y3 + y2 + y1)/4.0;
            let b = (y4 + y3 - y2 - y1)/4.0;
            let c = (y4 - y3 + y2 - y1)/4.0;
            let d = (y4 - y3 - y2 + y1)/4.0;
            let avg_pb = (pb1 + pb2 + pb3 + pb4)/4.0;
            let avg_pr = (pr1 + pr2 + pr3 + pr4)/4.0;
            // add those values to a tuple
            let block = (a, b, c, d, avg_pb, avg_pr);
            // push tuple into the output array
            new_pixels.push(block);
        }
    }
    new_pixels
}

/// Returns a vector of tuples that contain the brightness, avg_pb, avg_pr 
/// which are expanded into 2xr blocks from the inpit vector
///
/// # Arguments:
/// * 'comp_pixels`: A vector of tuples that contain quantized values (a, b, c, d, avg_pb, avg_pr) as floats
/// * `comp_w`: The width of the compressed image
/// * 'comp_h': The height of the compressed image
pub fn un_block(comp_pixels: Vec<(f64, f64, f64, f64, f64, f64)>, comp_w: u32, comp_h: u32) -> Vec<(f64, f64, f64)> {
    // comp_w and comp_h are half the size of trim_w and trim_h
    let mut float_vid: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); (comp_w * comp_h * 4) as usize];

    for row in 0..comp_h {
        for col in 0..comp_w {
            // setting a, b, c, d, avg_pb, avg_pr equal to their corresponding tuple value
            let a = comp_pixels[(row * comp_w + col) as usize].0;
            let b = comp_pixels[(row * comp_w + col) as usize].1;
            let c = comp_pixels[(row * comp_w + col) as usize].2;
            let d = comp_pixels[(row * comp_w + col) as usize].3;
            let avg_pb = comp_pixels[(row * comp_w + col) as usize].4;
            let avg_pr = comp_pixels[(row * comp_w + col) as usize].5;
            // gets each y value from a, b, c, d
            let y1 = a - b - c + d;
            let y2 = a - b + c - d;
            let y3 = a + b - c - d;
            let y4 = a + b + c + d;
            // add the y value ans avg_pb/avg_pr to each decompressed pixel
            float_vid[((row*2) * (comp_w*2) + (col*2)) as usize] = (y1, avg_pb, avg_pr);
            float_vid[((row*2) * (comp_w*2) + (col*2) + 1) as usize] = (y2, avg_pb, avg_pr);
            float_vid[(((row*2) + 1) * (comp_w*2) + (col*2)) as usize] = (y3, avg_pb, avg_pr);
            float_vid[(((row*2) + 1) * (comp_w*2) + (col*2) + 1) as usize] = (y4, avg_pb, avg_pr);
        }
    }
    float_vid
}

/// Returns a vector tuples containing the quantized integers of the input vector
///
/// # Arguments:
/// * 'new_pixels`: A vector of tuples that contains the float values for a, b, c, d, avg_pb, avg_pr
/// * `comp_w`: The width of the compressed image
/// * 'comp_h': The height of the compressed image
pub fn quantization(new_pixels: Vec<(f64, f64, f64, f64, f64, f64)>, comp_w: u32, comp_h: u32) 
-> Vec<(u64, i64, i64, i64, u64, u64)> {
    let mut q_blocks:Vec<(u64, i64, i64, i64, u64, u64)> = vec![];

    for row in 0..comp_h {
        for col in 0..comp_w {
            // quantize each value
            let q_a = (new_pixels[(row * comp_w + col) as usize].0 * 511.0).round() as u64; 
            let q_b = (new_pixels[(row * comp_w + col) as usize].1.clamp(-0.3, 0.3) * 15.0).round() as i64; 
            let q_c = (new_pixels[(row * comp_w + col) as usize].2.clamp(-0.3, 0.3) * 15.0).round() as i64; 
            let q_d = (new_pixels[(row * comp_w + col) as usize].3.clamp(-0.3, 0.3) * 15.0).round() as i64; 
            let q_avg_pb = index_of_chroma(new_pixels[(row * comp_w + col) as usize].4 as f32) as u64; 
            let q_avg_pr = index_of_chroma(new_pixels[(row * comp_w + col) as usize].5 as f32) as u64; 
            // add the quantized values to a tuple
            let q_block = (q_a, q_b, q_c, q_d, q_avg_pb, q_avg_pr);
            // push the tuple into a vector
            q_blocks.push(q_block);
        }
    }
    q_blocks
}

/// Returns a vector of tuples which have the float values for a, b, c, d, avg_pb, avg_pr
///
/// # Arguments:
/// * 'q_blocks`: A vector of tuples which contains the quantized integer values a, b, c, d, avg_pb, avg_pr
/// * `comp_w`: The width of the compressed image
/// * 'comp_h': The height of the compressed image
pub fn rev_quantization(q_blocks: Vec<(u64, i64, i64, i64, u64, u64)>, comp_w: u32, comp_h: u32)
 -> Vec<(f64, f64, f64, f64, f64, f64)> {
    let mut new_pixel: Vec<(f64, f64, f64, f64, f64, f64)> = vec![];

    for row in 0..comp_h {
        for col in 0..comp_w {
            // get f64 values from all of the quantized values
            let a = q_blocks[(row * comp_w + col) as usize].0 as f64 / 511.0;
            let b = q_blocks[(row * comp_w + col) as usize].1 as f64 / 15.0;
            let c = q_blocks[(row * comp_w + col) as usize].2 as f64 / 15.0;
            let d = q_blocks[(row * comp_w + col) as usize].3 as f64 / 15.0;
            let avg_pb = chroma_of_index(q_blocks[(row * comp_w + col) as usize].4 as usize) as f64; 
            let avg_pr = chroma_of_index(q_blocks[(row * comp_w + col) as usize].5 as usize) as f64;
            // add f64 values to tuple
            let block = (a, b, c, d, avg_pb, avg_pr);
            // push tuple to vector
            new_pixel.push(block);
        }
    }
    new_pixel
}