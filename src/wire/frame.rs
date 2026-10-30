use std::io::{self, Error, ErrorKind, Read, Write};

const MAX_FRAME_SIZE: usize = 1024 * 1024; // 1 MiB

pub fn write_frame(writer: &mut impl Write, data: &[u8]) -> io::Result<()> {
    if data.len() > MAX_FRAME_SIZE {
        return Err(Error::new(ErrorKind::InvalidInput, "Frame too large"));
    }

    let length = (data.len() as u32).to_be_bytes();

    writer.write_all(&length)?;
    writer.write_all(data)?;

    Ok(())
}

pub fn read_frame(reader: &mut impl Read) -> io::Result<Option<Vec<u8>>> {
    let mut length = [0u8; 4];

    if reader.read(&mut length[..1])? == 0 {
        return Ok(None);
    }

    reader.read_exact(&mut length[1..])?;

    let size = u32::from_be_bytes(length) as usize;

    if size > MAX_FRAME_SIZE {
        return Err(io::Error::new(ErrorKind::InvalidData, "Frame too large"));
    }

    let mut data = vec![0u8; size];
    reader.read_exact(&mut data)?;

    Ok(Some(data))
}
