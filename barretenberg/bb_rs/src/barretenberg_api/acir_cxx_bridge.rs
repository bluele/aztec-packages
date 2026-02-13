use super::{traits::SerializeBuffer, Buffer};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("barretenberg/dsl/acir_proofs/c_bind.hpp");

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
    }
}

fn decode_nested_buffer(out_ptr: *mut u8) -> Result<Vec<u8>, String> {
    let first = unsafe {
        Buffer::from_ptr(out_ptr).map_err(|e| format!("failed to read outer buffer: {e:?}"))?
    };
    let first_bytes = first.to_vec();
    let second = unsafe {
        Buffer::from_ptr(first_bytes.as_ptr())
            .map_err(|e| format!("failed to read inner buffer: {e:?}"))?
    };
    Ok(second.to_vec())
}

pub fn acir_prove_ultra_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_zk_honk_cxx(
            input_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception in acir_prove_ultra_zk_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}

pub fn acir_prove_ultra_keccak_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_keccak_honk_cxx(
            input_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception in acir_prove_ultra_keccak_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}

pub fn acir_prove_ultra_keccak_zk_honk_safe(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let witness_buffer = witness_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_prove_ultra_keccak_zk_honk_cxx(
            input_buffer.as_ptr(),
            witness_buffer.as_ptr(),
            vkey_buf.as_ptr(),
            &mut out_ptr,
        )
        .map_err(|e| format!("C++ exception in acir_prove_ultra_keccak_zk_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}

pub fn acir_get_ultra_honk_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception in acir_write_vk_ultra_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}

pub fn acir_get_ultra_honk_keccak_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_keccak_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception in acir_write_vk_ultra_keccak_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}

pub fn acir_get_ultra_honk_keccak_zk_verification_key_safe(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    let input_buffer = constraint_system_buf.to_buffer();
    let mut out_ptr = std::ptr::null_mut();

    unsafe {
        ffi::acir_write_vk_ultra_keccak_zk_honk_cxx(input_buffer.as_ptr(), &mut out_ptr)
            .map_err(|e| format!("C++ exception in acir_write_vk_ultra_keccak_zk_honk: {e}"))?;
    }

    decode_nested_buffer(out_ptr)
}
