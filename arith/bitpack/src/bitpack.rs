/// Returns true if the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let lower: i64 = 1 << 63 >> 63 - (width - 1);
    let upper: i64 = !(1 << 63 >> 63 - (width - 1));
    n >= lower && n <= upper
}
/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    n <= (1 << width) - 1
}
/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    if width == 0 {
        0
    } else {
        let value = word << 64 - (width + lsb) >> 64 - width;
        u_to_i(value, width)
    }
}
/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    if width == 0 {
        0
    } else {
        word << 64 - (width + lsb) >> 64 - width    
    }
}
/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width) {
        let left_part = (word >> (width + lsb)) << (width + lsb);
        let right_part = if lsb != 0 {(word << (64 - lsb)) >> (64 - lsb)} else {0};
        let new_value = value << lsb;
        Some(new_value | left_part | right_part)
    } else {
        None
    }
}
/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None if the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    let u_val = i_to_u(value, width);
    if fitss(value, width) {
        Some(newu(word, width, lsb, u_val).unwrap())
    } else {
        None
    }
}

/// Returns the unsigned equivalent of the singed value 'value'
///
/// # Arguments:
/// * 'value`: A signed integer value
/// * `width`: the width of a bit field
pub fn i_to_u(value: i64, width: u64) -> u64 {
    if value >= 0 {
        value as u64
    } else {
        let pos_value = value + (1 << width);
        pos_value as u64
    }
}     

/// Returns the signed equivalent of the unsinged value 'value'
///
/// # Arguments:
/// * 'value`: An unsigned integer value
/// * `width`: the width of a bit field
pub fn u_to_i(value: u64, width: u64) -> i64 {
    if value < (1 << (width - 1)) {
        value as i64
    } else {
        let neg_value = (value as i64) - (1 << width);
        neg_value
    } 
}