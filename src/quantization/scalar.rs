use super::{Quantization, QuantizationError, StorageType};
use crate::models::common::quantize_to_u8_bits;
use crate::storage::Storage;
use half::f16;

#[derive(Debug)]
pub struct ScalarQuantization;

impl Quantization for ScalarQuantization {
    fn quantize(&self, vector: &[f32], storage_type: StorageType) -> Storage {
        match storage_type {
            StorageType::UnsignedByte => {
                let quant_vec: Vec<_> = vector.iter().map(|&x| (x * 255.0).round() as u8).collect();
                let mag = quant_vec.iter().map(|&x| x as u32 * x as u32).sum();
                Storage::UnsignedByte { mag, quant_vec }
            }
            StorageType::SubByte(resolution) => {
                let quant_vec: Vec<_> = quantize_to_u8_bits(vector, resolution);
                // TODO: compute magnitude
                let mag = 0;
                Storage::SubByte {
                    mag,
                    quant_vec,
                    resolution,
                }
            }
            StorageType::HalfPrecisionFP => {
                let quant_vec = vector.iter().map(|&x| f16::from_f32(x)).collect();
                let mag = vector.iter().map(|&x| x * x).sum();
                Storage::HalfPrecisionFP { mag, quant_vec }
            }
        }
    }

    fn train(&mut self, _vectors: &[&[f32]]) -> Result<(), QuantizationError> {
        // Scalar quantization doesn't require training
        Ok(())
    }
}
