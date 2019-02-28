use std::fmt;

pub fn sha1(input: &[u8]) -> SHA1Digest {
    let original_input_length = input.len();

    let mut input = padded(input.into());
    input.extend(((original_input_length*8) as u64).to_be_bytes().iter());

    let mut h0 = 0x67452301u32;
    let mut h1 = 0xEFCDAB89u32;
    let mut h2 = 0x98BADCFEu32;
    let mut h3 = 0x10325476u32;
    let mut h4 = 0xC3D2E1F0u32;

    for chunk in input.chunks(64) {
        let mut w: Vec<u32> = chunk.chunks(4).map(|word| to_u32(word)).collect();
        w.reserve(80);
        for i in 16..80 {
            w.insert(i, (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1));
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        for i in 0..80 {
            let round = i / 20;
            let (f, k) = match round {
                0 => (
                    (b & c) | ((!b) & d),
                    0x5A827999u32
                ),
                1 => (
                    b ^ c ^ d,
                    0x6ED9EBA1u32,
                ),
                2 => (
                    (b & c) | (b & d) | (c & d),
                    0x8F1BBCDCu32,
                ),
                _ => (
                    b ^ c ^ d,
                    0xCA62C1D6u32,
                )
            };
            let temp = a.rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    SHA1Digest::new([h0, h1, h2, h3, h4])
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
    u32::from_be_bytes(word)
}

pub struct SHA1Digest {
    values: [u32; 5],
}

impl SHA1Digest {
    pub fn new<T: Into<[u32; 5]>>(input: T) -> Self {
        SHA1Digest {
            values: input.into(),
        }
    }
}

impl fmt::Debug for SHA1Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.values {
            for b in value.to_be_bytes().iter() {
                write!(f, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_format() {
        let hash = SHA1Digest::new([
            0x7F2C9FDEu32, 0x3A1B5ED2u32, 0x5AE8D3FAu32, 0x9B7DD10Bu32, 0xB3B40D10u32
        ]);
        assert_eq!(format!("{:?}", hash), "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3");
    }

    #[test]
    fn test_sha1() {
        let hash = sha1(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(format!("{:?}", hash), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn test_sha1_little_change() {
        let hash = sha1(b"The quick brown fox jumps over the lazy cog");
        assert_eq!(format!("{:?}", hash), "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3");
    }

    #[test]
    fn test_sha1_empty() {
        let hash = sha1(b"");
        assert_eq!(format!("{:?}", hash), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_sha1_one_char() {
        let hash = sha1(b"a");
        assert_eq!(format!("{:?}", hash), "86f7e437faa5a7fce15d1ddcb9eaeaea377667b8");
    }
}
