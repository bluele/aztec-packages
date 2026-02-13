use super::{bindgen, models::Ptr, traits::SerializeBuffer, Buffer};
use std::env;
use std::ptr;

#[derive(Debug)]
pub struct CircuitSizes {
    pub total: u32,
    pub subgroup: u32,
}

pub unsafe fn get_circuit_sizes(constraint_system_buf: &[u8], recursive: bool) -> CircuitSizes {
    let mut total = 0;
    let mut subgroup = 0;
    let honk_recursion = true;
    bindgen::acir_get_circuit_sizes(
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        &recursive,
        &honk_recursion,
        &mut total,
        &mut subgroup,
    );
    CircuitSizes {
        total: total.to_be(),
        subgroup: subgroup.to_be(),
    }
}

pub fn acir_prove_ultra_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
    slow_low_memory: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    acir_set_slow_low_memory(slow_low_memory);
    acir_set_storage_budget(max_storage_usage.unwrap_or(0));

    super::acir_cxx_bridge::acir_prove_ultra_honk_safe(constraint_system_buf, witness_buf, vkey_buf)
}

pub fn acir_prove_ultra_keccak_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
    slow_low_memory: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    acir_set_slow_low_memory(slow_low_memory);
    acir_set_storage_budget(max_storage_usage.unwrap_or(0));

    super::acir_cxx_bridge::acir_prove_ultra_keccak_honk_safe(
        constraint_system_buf,
        witness_buf,
        vkey_buf,
    )
}

pub fn acir_prove_ultra_keccak_zk_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
    slow_low_memory: bool,
    max_storage_usage: Option<u64>,
) -> Result<Vec<u8>, String> {
    acir_set_slow_low_memory(slow_low_memory);
    acir_set_storage_budget(max_storage_usage.unwrap_or(0));

    super::acir_cxx_bridge::acir_prove_ultra_keccak_zk_honk_safe(
        constraint_system_buf,
        witness_buf,
        vkey_buf,
    )
}

pub fn acir_get_ultra_honk_verification_key(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    super::acir_cxx_bridge::acir_get_ultra_honk_verification_key_safe(constraint_system_buf)
}

pub fn acir_get_ultra_honk_keccak_verification_key(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    super::acir_cxx_bridge::acir_get_ultra_honk_keccak_verification_key_safe(constraint_system_buf)
}

pub fn acir_get_ultra_honk_keccak_zk_verification_key(
    constraint_system_buf: &[u8],
) -> Result<Vec<u8>, String> {
    super::acir_cxx_bridge::acir_get_ultra_honk_keccak_zk_verification_key_safe(
        constraint_system_buf,
    )
}

pub unsafe fn acir_verify_ultra_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> bool {
    let mut result = false;
    bindgen::acir_verify_ultra_zk_honk(
        proof_buf.to_buffer().as_ptr(),
        vkey_buf.as_ptr(),
        &mut result,
    );
    result
}

pub unsafe fn acir_verify_ultra_keccak_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> bool {
    let mut result = false;
    bindgen::acir_verify_ultra_keccak_honk(
        proof_buf.to_buffer().as_ptr(),
        vkey_buf.as_ptr(),
        &mut result,
    );
    result
}

pub unsafe fn acir_verify_ultra_keccak_zk_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> bool {
    let mut result = false;
    bindgen::acir_verify_ultra_keccak_zk_honk(
        proof_buf.to_buffer().as_ptr(),
        vkey_buf.as_ptr(),
        &mut result,
    );
    result
}

pub unsafe fn acir_prove_and_verify_ultra_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
) -> bool {
    let mut result = false;
    bindgen::acir_prove_and_verify_ultra_honk(
        constraint_system_buf.to_buffer().as_ptr(),
        witness_buf.to_buffer().as_ptr(),
        &mut result,
    );
    result
}

pub unsafe fn acir_serialize_proof_into_fields(
    acir_composer_ptr: &mut Ptr,
    proof_buf: &[u8],
    num_inner_public_inputs: u32,
) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_serialize_proof_into_fields(
        acir_composer_ptr,
        proof_buf.to_buffer().as_ptr(),
        &num_inner_public_inputs.to_be(),
        &mut out_ptr,
    );
    Buffer::from_ptr(out_ptr).unwrap().to_vec()
}

pub unsafe fn acir_serialize_verification_key_into_fields(
    acir_composer_ptr: &mut Ptr,
) -> (Vec<u8>, [u8; 32]) {
    let mut out_vkey = ptr::null_mut();
    let mut out_key_hash = [0; 32];
    bindgen::acir_serialize_verification_key_into_fields(
        acir_composer_ptr,
        &mut out_vkey,
        out_key_hash.as_mut_ptr(),
    );
    (Buffer::from_ptr(out_vkey).unwrap().to_vec(), out_key_hash)
}

pub fn acir_set_slow_low_memory(enabled: bool) {
    if enabled {
        env::set_var("BB_SLOW_LOW_MEMORY", "1");
    } else {
        env::remove_var("BB_SLOW_LOW_MEMORY");
    }
}

pub fn acir_get_slow_low_memory() -> bool {
    env::var("BB_SLOW_LOW_MEMORY").map_or(false, |val| val == "1")
}

pub fn acir_set_storage_budget(max_bytes: u64) {
    if max_bytes == 0 {
        env::remove_var("BB_STORAGE_BUDGET");
        return;
    }

    // Use the max bytes directly for better precision
    env::set_var("BB_STORAGE_BUDGET", max_bytes.to_string());

    // This could be a way to do it, but it's not as precise
    // as the rounding gets too rough in the gigabytes
    /*if max_bytes < 1024 {
        env::set_var("BB_STORAGE_BUDGET", max_bytes.to_string());
    } else if max_bytes < 1024 * 1024 {
        let formatted_max_bytes = format!("{}k", max_bytes / 1024);
        env::set_var("BB_STORAGE_BUDGET", formatted_max_bytes);
    } else if max_bytes < 1024 * 1024 * 1024 {
        let formatted_max_bytes = format!("{}m", max_bytes / 1024 / 1024);
        env::set_var("BB_STORAGE_BUDGET", formatted_max_bytes);
    } else {
        let formatted_max_bytes = format!("{}g", max_bytes / 1024 / 1024 / 1024);
        env::set_var("BB_STORAGE_BUDGET", formatted_max_bytes);
    }*/
}

pub fn acir_set_storage_budget_from_string(budget_str: &str) {
    env::set_var("BB_STORAGE_BUDGET", budget_str);
}
