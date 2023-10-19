use bitpack::bitpack::{gets, getu, newu, news};

/// Returns a vector of codewords that represent the compressed pixels of an image
///
/// # Arguments:
/// * 'q_blocks`: A vector of tuples that contain quantized values (a, b, c, d, avg_pb, avg_pr)
/// * `comp_w`: The width of the compressed image
/// * 'comp_h': The height of the compressed image
pub fn pack_words(q_blocks: Vec<(u64, i64, i64, i64, u64, u64)>, comp_w: u32, comp_h: u32) -> Vec<[u8;4]> {
    let mut codewords: Vec<[u8;4]> = vec![];
    for row in 0..comp_h {
        for col in 0..comp_w {
            let mut codeword: u64 = 0;
            codeword = newu(codeword, 9, 23, q_blocks[(row * comp_w + col) as usize].0).unwrap();
            codeword = news(codeword, 5, 18, q_blocks[(row * comp_w + col) as usize].1).unwrap();
            codeword = news(codeword, 5, 13, q_blocks[(row * comp_w + col) as usize].2).unwrap();
            codeword = news(codeword, 5, 8, q_blocks[(row * comp_w + col) as usize].3).unwrap();
            codeword = newu(codeword, 4, 4, q_blocks[(row * comp_w + col) as usize].4).unwrap();
            codeword = newu(codeword, 4, 0, q_blocks[(row * comp_w + col) as usize].5).unwrap();

            codewords.push((codeword as u32).to_be_bytes());
        }
    }
    codewords
}

/// Returns a vector of tuples that contain the qunatized values (a, b, c, d, avg_pb, avg_pr)
///
/// # Arguments:
/// * 'codewords`: A vector of codewords that represent the values (a, b, c, d, avg_pb, avg_pr) 
/// for each pixel in a 32-bit integer thats boken up into 4 u8 values. 
/// * `comp_w`: The width of the compressed image
/// * 'comp_h': The height of the compressed image
pub fn unpack_words(codewords: &Vec<[u8;4]>, comp_w: u32, comp_h: u32) -> Vec<(u64, i64, i64, i64, u64, u64)> {
    let mut q_blocks: Vec<(u64, i64, i64, i64, u64, u64)> = vec![];
    for row in 0..comp_h {
        for col in 0..comp_w {
            let codeword = u32::from_be_bytes(codewords[(row * comp_w + col) as usize]);
            let q_a = getu(codeword as u64, 9, 23);
            let q_b = gets(codeword as u64, 5, 18);
            let q_c = gets(codeword as u64, 5, 13);
            let q_d = gets(codeword as u64, 5, 8);
            let q_avg_pb = getu(codeword as u64, 4, 4);
            let q_avg_pr = getu(codeword as u64, 4, 0);

            let q_block = (q_a, q_b, q_c, q_d, q_avg_pb, q_avg_pr);

            q_blocks.push(q_block);
        }
    }
    q_blocks
}