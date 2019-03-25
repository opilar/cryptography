use std::fmt;

pub fn md5_rounds(input: &[u8], rounds: usize) -> MD5Digest {
    let original_input_length = input.len();

    let mut input = padded(input.into());
    input.extend(((original_input_length*8) as u64).to_le_bytes().iter());

    let mut a0 = 0x67452301u32;
    let mut b0 = 0xefcdab89u32;
    let mut c0 = 0x98badcfeu32;
    let mut d0 = 0x10325476u32;

    for chunk in input.chunks(64) {
        let m: Vec<u32> = chunk.chunks(4).map(|word| to_u32(word)).collect();
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;
        for k in 0..rounds {
            let i = k % 64;
            let round = i / 16;
            let (mut f, g) = match round {
                0 => (
                    (b & c) | ((!b) & d),
                    i
                ),
                1 => (
                    (d & b) | ((!d) & c),
                    (i*5 + 1) % 16,
                ),
                2 => (
                    b ^ c ^ d,
                    (3*i + 5) % 16,
                ),
                _ => (
                    c ^ (b | (!d)),
                    (7*i) % 16,
                )
            };
            f = f.wrapping_add(a).wrapping_add(PRECOMPILED_TABLE[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(SHIFTS[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    MD5Digest::new([a0, b0, c0, d0])
}

pub fn md5(input: &[u8]) -> MD5Digest {
    md5_rounds(input, 64)
}

fn padded(mut input: Vec<u8>) -> Vec<u8> {
    input.push(0b1000_0000);

    while input.len() % 64 != 56 {
        input.push(0);
    }

    input
}

fn to_u32(word_slice: &[u8]) -> u32
{
    let mut iter = word_slice.iter();
    let word: [u8; 4] = [
        *iter.next().unwrap(),
        *iter.next().unwrap(),
        *iter.next().unwrap(),
        *iter.next().unwrap(),
    ];
    u32::from_le_bytes(word)
}

pub struct MD5Digest {
    values: [u32; 4],
}

impl MD5Digest {
    pub fn new<T: Into<[u32; 4]>>(input: T) -> Self {
        MD5Digest {
            values: input.into(),
        }
    }

    pub fn diff_with(&self, other: &MD5Digest) -> u32 {
        self.values.iter().zip(other.values.iter())
            .fold(0, |sum, pair| sum + diff(*pair.0, *pair.1))
    }
}

fn diff(first: u32, second: u32) -> u32 {
    (first ^ second).count_ones()
}

impl fmt::Debug for MD5Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.values {
            for b in value.to_le_bytes().iter() {
                write!(f, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

static SHIFTS: [u32; 64] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

static PRECOMPILED_TABLE: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_debug_format() {
        let hash = MD5Digest::new([
            u32::from_be(0x9e107d9d), u32::from_be(0x372bb682),
            u32::from_be(0x6bd81d35), u32::from_be(0x42a419d6)
        ]);
        assert_eq!(format!("{:?}", hash), "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_md5_simple() {
        let hash = md5(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(format!("{:?}", hash), "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_md5_little_change() {
        let hash = md5(b"The quick brown fox jumps over the lazy dog.");
        assert_eq!(format!("{:?}", hash), "e4d909c290d0fb1ca068ffaddf22cbd0");
    }

    #[test]
    fn test_md5_empty() {
        let hash = md5(b"");
        assert_eq!(format!("{:?}", hash), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_md5_one_char() {
        let hash = md5(b"a");
        assert_eq!(format!("{:?}", hash), "0cc175b9c0f1b6a831c399e269772661");
    }
}
