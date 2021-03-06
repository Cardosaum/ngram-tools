use std::fmt;

#[derive(Clone, Copy)]
enum CharType {
    Letter(u8),
    WordSep,
    // PhraseSep,
    SentenceSep,
    Join,
    Skip,
}

impl fmt::Debug for CharType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharType::Letter(a) => write!(f, "Letter({} ({}))", *a as char, a),
            CharType::WordSep => write!(f, "WordSep"),
            // CharType::PhraseSep => write!(f, "PhraseSep"),
            CharType::SentenceSep => write!(f, "SentenceSep"),
            CharType::Join => write!(f, "Join"),
            CharType::Skip => write!(f, "Skip"),
        }
    }
}

pub fn normalize_ascii(text: &Vec<u8>, keep_commas: bool) -> Vec<u8> {
    let mut write_buffer = Vec::with_capacity(text.len());
    let mut t1: CharType = CharType::Skip;
    let mut t2: CharType;
    for this_char in text.iter() {
        t2 = match this_char {
            b'A'...b'Z' => CharType::Letter(*this_char + 32),
            b'a'...b'z' | b'\'' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => CharType::Letter(*this_char),
            b' ' | b'\t' | b'\n' | b'"' | b'(' | b')' | b',' => CharType::WordSep,
            // b',' => CharType::PhraseSep,
            b'!' | b'?' | b'.' | b';' | b':' => CharType::SentenceSep,
            b'-' => CharType::Join,
            _ => CharType::Skip,
        };

        // println!(
        //     "{} ({}) {:?} {:?}",
        //     char::from(*this_char),
        //     this_char,
        //     t1,
        //     t2
        // );

        match (t1, t2) {
            (CharType::Skip, _) => {
                t1 = t2;
            }
            (CharType::Letter(a), _) => {
                t1 = t2;
                write_buffer.push(a);
            }
            (CharType::WordSep, CharType::Letter(_)) => {
                t1 = t2;
                write_buffer.push(b' ');
            }
            (CharType::WordSep, CharType::SentenceSep) => {
                t1 = CharType::SentenceSep;
            }
            // (CharType::PhraseSep, CharType::Letter(_)) => {
            //     t1 = t2;
            //     if keep_commas {
            //         write_buffer.push(b' ');
            //         write_buffer.push(b',');
            //         write_buffer.push(b' ');
            //     }
            // }
            // (CharType::PhraseSep, CharType::SentenceSep) => {
            //     t1 = t2;
            // }
            (CharType::SentenceSep, CharType::Letter(_)) => {
                t1 = t2;
                write_buffer.push(b'\n');
            }
            (CharType::Join, CharType::Letter(_)) => {
                t1 = t2;
                write_buffer.push(b' ');
            }
            (_, _) => (),
        }
    }

    match t1 {
        CharType::Letter(a) => write_buffer.push(a),
        CharType::SentenceSep => write_buffer.push(b'\n'),
        _ => (),
    }

    return write_buffer;
}
