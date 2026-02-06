use std::num::NonZeroU32;
use thiserror::Error;

use syro_sys::*;

pub const OUTPUT_SAMPLE_RATE: u32 = 44_100;

pub const MAX_SLOT: u32 = 99;

const COMPRESSION_QUALITY: u32 = 16;

#[derive(Clone, Debug, Error)]
pub enum SyroError {
    #[error("Invalid Volca Sample slot {0}, must be 0-{MAX_SLOT}")]
    InvalidSlot(u32),

    #[error("{0} failed with status {1}")]
    VolcaSampleOp(&'static str, SyroStatus),
}

pub fn encode(
    src_audio: &[i16],
    src_rate: NonZeroU32,
    dst_slot: u32,
) -> Result<Vec<i16>, SyroError> {
    let mut syro_data = prepare_syrodata(src_audio, src_rate, dst_slot)?;
    let mut handle: SyroHandle = std::ptr::null_mut();
    let mut num_frames: u32 = 0;

    let status =
        unsafe { SyroVolcaSample_Start(&mut handle, &mut syro_data, 1, 0, &mut num_frames) };

    if status != SyroStatus_Status_Success {
        return Err(SyroError::VolcaSampleOp("SyroVolcaSample_Start", status));
    }

    let mut output: Vec<i16> = Vec::with_capacity((num_frames * 2) as usize);
    for _ in 0..num_frames {
        let mut left: i16 = 0;
        let mut right: i16 = 0;

        let status = unsafe { SyroVolcaSample_GetSample(handle, &mut left, &mut right) };
        if status != SyroStatus_Status_Success {
            return Err(SyroError::VolcaSampleOp(
                "SyroVolcaSample_GetSample",
                status,
            ));
        }

        output.push(left);
        output.push(right);
    }

    let status = unsafe { SyroVolcaSample_End(handle) };
    if status != SyroStatus_Status_Success {
        return Err(SyroError::VolcaSampleOp("SyroVolcaSample_End", status));
    }

    Ok(output)
}

fn prepare_syrodata(
    src_audio: &[i16],
    src_rate: NonZeroU32,
    dst_slot: u32,
) -> Result<SyroData, SyroError> {
    if dst_slot > MAX_SLOT {
        return Err(SyroError::InvalidSlot(dst_slot));
    }

    let bytes: Vec<u8> = src_audio
        .iter()
        .flat_map(|&sample| sample.to_le_bytes())
        .collect();

    Ok(SyroData {
        DataType: SyroDataType_DataType_Sample_Compress,
        pData: bytes.as_ptr() as *mut u8,
        Number: dst_slot,
        Size: bytes.len() as u32,
        Quality: COMPRESSION_QUALITY,
        Fs: src_rate.get(),
        SampleEndian: Endian_LittleEndian,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_syrodata_valid_slots_returns_ok() {
        for slot in 0..=MAX_SLOT {
            let result = prepare_syrodata(&[1, 2, 3], NonZeroU32::new(44100).unwrap(), slot);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn prepare_syrodata_invalid_slot_returns_err() {
        let result = prepare_syrodata(&[1, 2, 3], NonZeroU32::new(44100).unwrap(), 100);
        assert!(matches!(result.unwrap_err(), SyroError::InvalidSlot(slot) if slot == 100));
    }
}
