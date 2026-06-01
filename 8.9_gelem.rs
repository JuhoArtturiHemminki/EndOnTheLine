        }

        let mut adaptive_bias = v_chunk[0];
        let alpha = 0.0625f32;
        let inv_simd_width = 0.0625f32;

        for (i, simd_chunk) in v_chunk.chunks_exact(simd_width).enumerate() {
            let voltage_vector = f32x16::from_slice(simd_chunk);

            let threshold_vector = f32x16::splat(adaptive_bias);
            let mask = voltage_vector.simd_gt(threshold_vector);

            m_chunk[i] = mask.to_bitmask() as u16;

            let chunk_sum = voltage_vector.reduce_sum();
            let chunk_avg = chunk_sum * inv_simd_width;
            adaptive_bias = (alpha * chunk_avg) + ((1.0 - alpha) * adaptive_bias);
        }
    });
}
