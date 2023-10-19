use crate::pixel::{trim_image, rgb_to_float, float_to_rgb, read_image, write_image, float_to_vid, vid_to_float};
use crate::block::{get_block, un_block, quantization, rev_quantization};
use crate::codeword::{pack_words, unpack_words};

pub fn compress(filename: &str) {
    let decomp_img = read_image(filename);
    let trim_w = decomp_img.width & !1_u32;
    let trim_h = decomp_img.height & !1_u32;
    let comp_w = trim_w / 2;
    let comp_h = trim_h / 2;
    let denom = decomp_img.denominator;
    
    let rgb_vec = trim_image(decomp_img);
    let float_vec = rgb_to_float(rgb_vec, denom);
    let vid_vec = float_to_vid(float_vec);
    let block_vec = get_block(vid_vec, trim_w, trim_h);
    let q_block_vec = quantization(block_vec, comp_w, comp_h);
    let codewords = pack_words(q_block_vec, comp_w, comp_h);
    csc411_rpegio::output_rpeg_data(&codewords, trim_w, trim_h);
}

pub fn decompress(filename: &str) {
    let comp_img = csc411_rpegio::read_in_rpeg_data(Some(filename));
    let codewords = &comp_img.as_ref().unwrap().0;
    let comp_w = comp_img.as_ref().unwrap().1 / 2;
    let comp_h = comp_img.as_ref().unwrap().2 / 2;
    let q_block_vec = unpack_words(codewords, comp_w, comp_h);
    let block_vec = rev_quantization(q_block_vec, comp_w, comp_h);
    let vid_vec = un_block(block_vec, comp_w, comp_h);
    let float_vec = vid_to_float(vid_vec);
    let rgb_vec = float_to_rgb(float_vec);
    write_image(rgb_vec, comp_w * 2, comp_h * 2);
}