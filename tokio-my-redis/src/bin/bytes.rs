use std::io;
use std::io::{Cursor, Seek, SeekFrom, Write};
use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    bytes();
    bytes_bufmut();
    cursor();
}

// BytesMut实现了Buf，BufMut特征
// Bytes实现了Buf 特征
fn bytes() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    let other = buf.split();

    assert!(buf.is_empty());
    assert_eq!(1013, buf.capacity());

    assert_eq!(11, other.capacity());
    assert_eq!(other, b"hello world"[..]);
}

// &mut [u8], Vec<u8> 实现了BufMut
fn bytes_bufmut() {
    let mut buf = vec![];

    buf.put_u8(b'h');
    buf.put(&b"ello"[..]);
    buf.put(&b" world"[..]);

    assert_eq!(buf, b"hello world");
}

// io::cursor 为 AsRef<[u8]> 实现了Seek，Read，BufRead特征
// 为 &mut [u8]等实现了Write特征
fn cursor() {
    let mut buff = Cursor::new(vec![0; 15]);

    // 针对 &mut [u8] 可以定位写
    write_ten_bytes_at_end(&mut buff).unwrap();

    assert_eq!(&buff.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
// a library function we've written
fn write_ten_bytes_at_end<W: Write + Seek>(mut writer: W) -> io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;

    for i in 0..10 {
        writer.write(&[i])?;
    }

    // all went well
    Ok(())
}
