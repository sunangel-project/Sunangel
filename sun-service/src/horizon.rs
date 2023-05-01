use std::f64::consts::PI;

use anyhow::{anyhow, Error};

const HORIZON_SAMPLES: usize = 1024;
const HORIZON_ANGLE: f64 = 2. * PI / (HORIZON_SAMPLES as f64);

const BYTES_IN_F64: usize = 8;

pub struct Horizon {
    altitudes: [f64; HORIZON_SAMPLES],
}

impl Horizon {
    pub fn decode(input: Vec<u8>) -> Result<Self, Error> {
        if input.len() != HORIZON_SAMPLES * BYTES_IN_F64 {
            return Err(anyhow!(
                "Expected input to have {} bytes, had {}",
                HORIZON_SAMPLES * BYTES_IN_F64,
                input.len()
            ));
        }

        let altitudes: Vec<f64> = input
            .chunks(BYTES_IN_F64)
            .map(|bytes| {
                let bytes = <&[u8; BYTES_IN_F64]>::try_from(bytes)
                    .expect("Chunk of input did not have correct size. Sould never happen");
                f64::from_le_bytes(*bytes)
            })
            .collect();

        let altitudes = <[f64; HORIZON_SAMPLES]>::try_from(altitudes)
            .expect("Altitudes result array did not have correct size. Sjould never happen");

        Ok(Horizon { altitudes })
    }
}
