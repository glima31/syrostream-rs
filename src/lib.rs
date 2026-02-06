use syro_sys::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SyroError {
    #[error("SyroVolcaSample_Start failed with status {0}")]
    VolcaSampleStart(SyroStatus),

    #[error("SyroVolcaSample_GetSample failed with status {0}")]
    VolcaSampleGetSample(SyroStatus),

    #[error("SyroVolcaSample_End failed with status {0}")]
    VolcaSampleEnd(SyroStatus),
}
