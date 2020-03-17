#![feature(with_options)]

use std::io::{Write, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;

use rand::Rng;

const DATA_PATTERN: &[u32; 27] = &[
    0x55_55_55, 0xAA_AA_AA, 0x92_49_24, 0x49_24_92, 0x24_92_49, 0x00_00_00, 0x11_11_11, 0x22_22_22,
    0x33_33_33, 0x55_55_55, 0x66_66_66, 0x77_77_77, 0x88_88_88, 0x99_99_99, 0xAA_AA_AA, 0xBB_BB_BB,
    0xCC_CC_CC, 0xDD_DD_DD, 0xEE_EE_EE, 0xFF_FF_FF, 0x92_49, 24, 0x49_24_92, 0x24_92_49,
    0x6D_B6_DB, 0xB6_DB_6D, 0xDB_6D_B6,
];

/// Applies the gutmann algorithm to an file
pub fn apply_to_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut file = File::with_options().read(true).write(true).append(false).open(path)?;
    let size = file.metadata()?.len() as usize;
    let mut rng = rand::thread_rng();

    for _ in 0..4 {
        let random: u32 = rng.gen();
        write_pattern(&mut file, size, random)?;
        file.flush()?;
        file.seek(SeekFrom::Start(0))?;
    }

    for pattern in DATA_PATTERN {
        write_pattern(&mut file, size, *pattern)?;
        file.flush()?;
        file.seek(SeekFrom::Start(0))?;
    }

    for _ in 0..4 {
        let random: u32 = rng.gen();
        write_pattern(&mut file, size, random)?;
        file.flush()?;
        file.seek(SeekFrom::Start(0))?;
    }

    Ok(())
}

fn write_pattern<W: Write>(write: &mut W, size: usize, pattern: u32) -> std::io::Result<()> {
    let bytes: [u8; 4] = unsafe { std::mem::transmute(pattern.to_be()) };
    let remaining = size % 4;
    for _ in 0..(size / 4) {
        write.write(&bytes)?;
    }

    for i in 0..remaining {
        write.write(&[bytes[i]])?;
    }

    Ok(())
}