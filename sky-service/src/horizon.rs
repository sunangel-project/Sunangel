use std::f64::consts::PI;

use anyhow::{anyhow, Error};

use crate::angle::AngleExtensions;

pub const HORIZON_SAMPLES: usize = 1024;
pub const HORIZON_ANGLE: f64 = 2. * PI / (HORIZON_SAMPLES as f64);

const BYTES_IN_F64: usize = 8;

#[derive(Debug)]
pub struct Horizon {
    altitudes: [f64; HORIZON_SAMPLES],
}

impl Horizon {
    pub fn new(altitudes: [f64; HORIZON_SAMPLES]) -> Self {
        Self { altitudes }
    }

    pub fn altitude_at(&self, pos: f64) -> f64 {
        let pos = pos.normalize_radians();

        let left = (pos / HORIZON_ANGLE).floor() as usize;
        let right = if left < HORIZON_SAMPLES - 1 {
            left + 1
        } else {
            0
        };

        let left_height = self.altitudes[left];
        let right_height = self.altitudes[right];

        let offset = pos - left as f64 * HORIZON_ANGLE;

        left_height + offset * (right_height - left_height) / HORIZON_ANGLE
    }
}

impl TryFrom<Vec<u8>> for Horizon {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != HORIZON_SAMPLES * BYTES_IN_F64 {
            return Err(anyhow!(
                "Expected input to have {} bytes, had {}",
                HORIZON_SAMPLES * BYTES_IN_F64,
                value.len()
            ));
        }

        let altitudes: Vec<f64> = value
            .chunks(BYTES_IN_F64)
            .map(|bytes| {
                let bytes: &[u8; BYTES_IN_F64] = bytes
                    .try_into()
                    .expect("Chunk of input did not have correct size. Should never happen");
                f64::from_le_bytes(*bytes)
            })
            .collect();

        let altitudes: [f64; HORIZON_SAMPLES] = altitudes
            .try_into()
            .expect("Altitudes result array did not have correct size. Should never happen");

        Ok(Horizon { altitudes })
    }
}
