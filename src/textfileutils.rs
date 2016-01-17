use std::io;
use std::io::{SeekFrom, BufReader, Cursor};
use std::io::prelude::*;

fn get_first_line_after<'a, R: Read + Seek>(reader: &mut BufReader<R>, from: usize) -> String {
    find_new_line_pos(reader, from).unwrap()
}

const SIZE: usize = 256;
// it is able to find strings up-to 256 elements wide
fn find_new_line_pos<'a, R: Read + Seek>(reader: &mut BufReader<R>, from: usize) -> Option<String> {
    let before = match from.checked_sub(SIZE) {
        None => 0,
        Some(x) => x
    };
    reader.seek(SeekFrom::Start(before as u64)).unwrap();
    let mut buf: [u8;2*SIZE] = [0; 2*SIZE];
    let len = reader.read(&mut buf).unwrap();
    let bufs = &buf[0..len];
    print!("{:?}", String::from_utf8_lossy(bufs));
    let last_before = bufs.iter().enumerate().rposition(|(i, x)| *x==b'\n' && (i + before) < from);
    let last_after = bufs.iter().enumerate().position(|(i, x)| *x==b'\n' && (i + before) >= from);
    let str_before = match last_before {
        None => String::from_utf8_lossy(&buf[0..from-before]).into_owned(),
        Some(pos) => String::from_utf8_lossy(&buf[pos+1..from-before]).into_owned()
    };
    let str_after = match last_after {
        None => String::from_utf8_lossy(&buf[from-before..len]).into_owned(),
        Some(pos) => String::from_utf8_lossy(&buf[from-before..pos]).into_owned()
    };
    Some(str_before + &str_after)
}

#[test]
fn find_new_line_pos_works() {
    let data = String::from("some\nother\nline");
    let bytes = data.as_bytes();
    let mut test_data = BufReader::new(Cursor::new(bytes));
    {
        let pos = find_new_line_pos(&mut test_data, 5);
        assert_eq!(Some("other".to_string()),pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 1);
        assert_eq!(Some("some".to_string()), pos)
    }
    {
        let pos = find_new_line_pos(&mut test_data, 4);
        assert_eq!(Some("some".to_string()), pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 10);
        assert_eq!(Some("other".to_string()), pos);
    }
    {
        let pos = find_new_line_pos(&mut test_data, 11);
        assert_eq!(Some("line".to_string()), pos);
    }
}
