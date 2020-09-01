use std::io;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

const VERSION: u16 = 1;

pub fn write(target: &mut impl Write) -> io::Result<()> {
    // file header
    write!(target, "MCBM")?;
    target.write_u16::<LittleEndian>(VERSION)?;

    Ok(())
}