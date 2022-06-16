//! Classes needed for affix attributes

enum AffixClass {
    Pfx,
    Sfx,
    Rep,
}

struct Affix {
    class: AffixClass,
}

#[derive(PartialEq)]
pub enum Encoding {
    Utf8,            // UTF-8
    Iso8859t1,       // ISO8859-1
    Iso8859t10,      // ISO8859-10
    Iso8859t13,      // ISO8859-13
    Iso8859t15,      // ISO8859-15
    Koi8r,           // KOI8-R
    Koi8u,           // KOI8-U
    Cp1251,          // cp1251
    IsciiDevanagari, // ISCII-DEVANAGARI
}

/// Raise when
#[inline]
pub fn match_encoding(s: &str) -> Result<Encoding, &'static str> {
    let mut split = s.split_whitespace();

    // "SET"
    let _ = split.next();
    // Calling function should never give us something empty
    let name = split
        .next()
        .expect("No encoding listed after \"SET\"")
        .trim();

    match name {
        "UTF-8" => Ok(Encoding::Utf8),
        "ISO8859-1" => Ok(Encoding::Iso8859t1),
        "ISO8859-10" => Ok(Encoding::Iso8859t10),
        "ISO8859-13" => Ok(Encoding::Iso8859t13),
        "ISO8859-15" => Ok(Encoding::Iso8859t15),
        "KOI8-R" => Ok(Encoding::Koi8r),
        "KOI8-U" => Ok(Encoding::Koi8u),
        "cp1251" => Ok(Encoding::Cp1251),
        "ISCII-DEVANAGARI" => Ok(Encoding::IsciiDevanagari),
        _ => Err("Encoding not found"),
    }
}

struct AffixData {
    // encoding:
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_encoding() {
        assert!(match_encoding("SET UTF-8").unwrap() == Encoding::Utf8);
        assert!(match_encoding("SET ISO8859-1").unwrap() == Encoding::Iso8859t1);
        assert!(match_encoding("SET ISO8859-10").unwrap() == Encoding::Iso8859t10);
        assert!(match_encoding("SET ISO8859-13").unwrap() == Encoding::Iso8859t13);
        assert!(match_encoding("SET ISO8859-15").unwrap() == Encoding::Iso8859t15);
        assert!(match_encoding("SET KOI8-R").unwrap() == Encoding::Koi8r);
        assert!(match_encoding("SET KOI8-U").unwrap() == Encoding::Koi8u);
        assert!(match_encoding("SET cp1251").unwrap() == Encoding::Cp1251);
        assert!(match_encoding("SET ISCII-DEVANAGARI").unwrap() == Encoding::IsciiDevanagari);

        // Test whitespace differences
        assert!(match_encoding(" SET\tUTF-8 ").unwrap() == Encoding::Utf8);
    }
}
