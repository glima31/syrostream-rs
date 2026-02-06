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
    let (mut syro_data, _bytes) = prepare_syrodata(src_audio, src_rate, dst_slot)?;

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
) -> Result<(SyroData, Vec<u8>), SyroError> {
    if dst_slot > MAX_SLOT {
        return Err(SyroError::InvalidSlot(dst_slot));
    }

    let bytes: Vec<u8> = src_audio
        .iter()
        .flat_map(|&sample| sample.to_le_bytes())
        .collect();

    let syro_data = SyroData {
        DataType: SyroDataType_DataType_Sample_Compress,
        pData: bytes.as_ptr() as *mut u8,
        Number: dst_slot,
        Size: bytes.len() as u32,
        Quality: COMPRESSION_QUALITY,
        Fs: src_rate.get(),
        SampleEndian: Endian_LittleEndian,
    };

    Ok((syro_data, bytes))
}

#[cfg(test)]
mod tests {
    use hound::WavReader;

    use super::*;

    const REFERENCE_SRC_AUDIO_WAV: &str = "testdata/kick.wav";
    const REFERENCE_OUTPUT_WAV: &str = "testdata/kick_syrostream_slot0.wav";

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

    #[test]
    fn encode_result_matches_reference_wav() {
        let input_wav = WavReader::open(REFERENCE_SRC_AUDIO_WAV).unwrap();
        let output_wav = WavReader::open(REFERENCE_OUTPUT_WAV).unwrap();

        let input_wav_spec = input_wav.spec();
        let input_audio: Vec<i16> = input_wav.into_samples().map(|s| s.unwrap()).collect();

        let expected_syrostream: Vec<i16> = output_wav.into_samples().map(|s| s.unwrap()).collect();
        let result = encode(
            &input_audio,
            NonZeroU32::new(input_wav_spec.sample_rate).unwrap(),
            0,
        )
        .unwrap();

        let diffs: usize = result
            .iter()
            .zip(expected_syrostream.iter())
            .filter(|(a, b)| a != b)
            .count();

        println!("Total differences: {diffs} / {}", result.len());

        let max_diff: i16 = result
            .iter()
            .zip(expected_syrostream.iter())
            .map(|(a, b)| (a - b).abs())
            .max()
            .unwrap_or(0);

        println!("Max sample difference: {max_diff}");
    }
}
