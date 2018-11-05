//! # Perceptual hash and distance module
//! Perceptual hashing is used to identify similar strings in a concise and fast way.
//! This is part of an algorithm laid out in: https://arxiv.org/ftp/arxiv/papers/1412/1412.5517.pdf
//! where image compression algorithms were altered to represent DNA strings.
//!

/// Compute the distance between two bytes using the hamming distance algorithm
///
#[allow(dead_code)]
fn byte_distance(b1 : u8, b2 : u8) -> u8 {
    let mut mask : u8 = 0x01;
    let mut distance : u8 = 0;

    // increment the mask to reveal if bit is flipped on at that location
    let mut index = 0;
    while index < 8 {
        if mask & b2 != mask & b1 {
            distance += 1;
        }
        mask <<= 1;
        index += 1;
    }
    println!("b1: {:#x}  b2: {:#x}", b1, b2);
    distance
}


/// Compute the hamming distance: the number of bit flips it takes for one string to match the other
/// # Example
/// ```
/// /*
///  * The distance between these two can be calculated by the number of bit flips.
///  * Since they're the same value until the last digit we just have to look at the last 1/2 byte
///  * 
///  * s1: 0001
///  * s2: 1001
///  * 
///  * In order to get s2 == s1, we need to do one bit flip.
///  * 
///  */
/// let s1 = "AAAA";  // 0x41414141
/// let s2 = "AAAI";  // 0x41414149
/// 
/// let d = distance(s1.as_bytes(), s2.as_bytes()); // 1
/// ```
/// 
/// # Panic
/// The function panics if the two strings are not the same length
///
#[allow(dead_code)]
pub fn distance(bytes1 : &[u8], bytes2 : &[u8]) -> u64 {

    // In order to compute Hamming Distance, each byte string must be same length 
    // (otherwise distance is infinite)
    if bytes1.len() != bytes2.len() {
        panic!("strings are not the same length");
    }
    let mut total_distance : u64 = 0;

    for i in 0..bytes1.len() {
        total_distance += u64::from(byte_distance(bytes1[i], bytes2[i]));
    }

    total_distance
}

/// Calculate the distance between two strings of 256 bits (u64).
/// Returns the number of bits which need to be flipped in order for
/// one string to match the other.
///
/// # Example
/// ```
/// let val1 :u64 = 0x0F;
/// let val2 :u64 = 0x08;
///
/// let d = distance_u64(val1, val2); // 3
///
/// ```
#[allow(dead_code)]
pub fn distance_u64(b1: u64, b2: u64) -> u8 {
    let mut mask : u64 = 0x01;
    let mut distance : u8 = 0;

    // increment the mask to reveal if bit is flipped on at that location
    let mut index = 0;
    while index < 64 {
        if mask & b2 != mask & b1 {
            distance += 1;
        }
        mask <<= 1;
        index += 1;
    }
    distance
}




/// Perceptual hash function. Takes a string of bytes as an argument
/// and returns a 256 bit value based on that string. Similar strings
/// SHOULD produce similar hashes. This is the underlying principle
/// of perceptual hashes.

#[allow(dead_code)]
pub fn hash(string : &[u8]) -> u64 {
    let mut hvalue : u64 = 0xAAAAAAAA;

    for byte in string {
        let mut mask = u64::from(*byte);
        hvalue ^= mask;
        hvalue += mask;
        hvalue <<= 8;
    }

    hvalue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_similarity() {
        let s1 = "1234567890";
        let s2 = "1234567870";

        let s1_hash = hash(s1.as_bytes());
        let s2_hash = hash(s2.as_bytes());

        let distance = distance_u64(s1_hash, s2_hash);
        println!("hash1: {:#x}\nhash2: {:#x}\ndistance: {}", s1_hash, s2_hash, distance);

    }

    #[test]
    fn byte_distance_functional() {

        // 00000000 00000001
        assert_eq!(byte_distance(0x00, 0x01), 1);

        // 00000000 11111111
        assert_eq!(byte_distance(0x00, 0xFF), 8);

        // 00000000 00000100
        assert_eq!(byte_distance(0x00, 0x04), 1);

        // 00000000 01010100
        assert_eq!(byte_distance(0x00, 0x54), 3);

        // 00000111 00001001
        assert_eq!(byte_distance(0x07, 0x09), 3);

        // 01010001 00110000 
        assert_eq!(byte_distance(0x41,0x30), 4);

    }

    #[test]
    fn distance_functional() {

        let value = distance("0001".as_bytes(),"0000".as_bytes());
        println!("testing: 0001 0000");
        assert_eq!(value, 1);

        let value = distance("000A".as_bytes(),"0000".as_bytes());
        println!("testing: 000A 0000");
        assert_eq!(value, 4);
    }

    #[test]
    #[should_panic]
    fn distance_safety() {
        let s1 = String::from("hello!");
        println!("testing: hello! 0000");
        let value = distance(s1.as_bytes(),"0000".as_bytes());
        assert_eq!(value, 6);
    }

}
