// Use cxx to safely handle C++ exceptions from existing functions

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("barretenberg/dsl/acir_proofs/c_bind.hpp");

        #[cxx_name = "acir_write_vk_ultra_honk"]
        unsafe fn acir_write_vk_ultra_honk_cxx(
            acir_vec: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;

        #[cxx_name = "acir_write_vk_ultra_keccak_honk"]
        unsafe fn acir_write_vk_ultra_keccak_honk_cxx(
            acir_vec: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;

        #[cxx_name = "acir_write_vk_ultra_keccak_zk_honk"]
        unsafe fn acir_write_vk_ultra_keccak_zk_honk_cxx(
            acir_vec: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;

        #[cxx_name = "acir_prove_ultra_zk_honk"]
        unsafe fn acir_prove_ultra_zk_honk_cxx(
            acir_vec: *const u8,
            witness_vec: *const u8,
            vk_buf: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;

        #[cxx_name = "acir_prove_ultra_keccak_honk"]
        unsafe fn acir_prove_ultra_keccak_honk_cxx(
            acir_vec: *const u8,
            witness_vec: *const u8,
            vk_buf: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;

        #[cxx_name = "acir_prove_ultra_keccak_zk_honk"]
        unsafe fn acir_prove_ultra_keccak_zk_honk_cxx(
            acir_vec: *const u8,
            witness_vec: *const u8,
            vk_buf: *const u8,
            out: *mut *mut u8,
        ) -> Result<()>;
    }
}

use super::traits::SerializeBuffer;

// Exception-safe wrapper functions
pub fn acir_get_ultra_honk_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}

pub fn acir_get_ultra_honk_keccak_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_keccak_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}

pub fn acir_get_ultra_honk_keccak_zk_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_keccak_zk_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}

pub fn acir_prove_ultra_zk_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let acir_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_zk_honk_cxx(
            acir_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}

pub fn acir_prove_ultra_keccak_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let acir_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_keccak_honk_cxx(
            acir_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}

pub fn acir_prove_ultra_keccak_zk_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    use super::Buffer;
    use std::ptr;

    let acir_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_keccak_zk_honk_cxx(
            acir_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception: {}", e))?;

        let buffer = Buffer::from_ptr(
            Buffer::from_ptr(out_ptr)
                .map_err(|e| format!("Failed to read output buffer: {:?}", e))?
                .to_vec()
                .as_slice()
                .as_ptr(),
        )
        .map_err(|e| format!("Failed to parse final buffer: {:?}", e))?;

        Ok(buffer.to_vec())
    }
}
