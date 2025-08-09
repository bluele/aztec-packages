use crate::barretenberg_api::acir::{
    acir_get_ultra_honk_keccak_verification_key, acir_get_ultra_honk_keccak_zk_verification_key,
    acir_get_ultra_honk_verification_key,
};

#[test]
fn test_safe_vk_generation_handles_exceptions() {
    // Trigger C++ exception with invalid input
    let invalid_input = vec![0u8; 10]; // Too small input

    // Test ultra_honk version
    match acir_get_ultra_honk_verification_key(&invalid_input) {
        Ok(_) => {
            assert!(false, "Unexpectedly succeeded with small input");
        }
        Err(e) => {
            println!("Successfully caught exception: {}", e);
            assert!(e.contains("C++ exception") || e.contains("error"));
        }
    }
}

#[test]
fn test_safe_vk_keccak_generation_handles_exceptions() {
    let invalid_input = vec![0u8; 10];

    match acir_get_ultra_honk_keccak_verification_key(&invalid_input) {
        Ok(_) => {
            assert!(false, "Unexpectedly succeeded with small input");
        }
        Err(e) => {
            println!("Successfully caught exception: {}", e);
            assert!(e.contains("C++ exception") || e.contains("error"));
        }
    }
}

#[test]
fn test_safe_vk_keccak_zk_generation_handles_exceptions() {
    let invalid_input = vec![0u8; 10];

    match acir_get_ultra_honk_keccak_zk_verification_key(&invalid_input) {
        Ok(_) => {
            assert!(false, "Unexpectedly succeeded with small input");
        }
        Err(e) => {
            println!("Successfully caught exception: {}", e);
            assert!(e.contains("C++ exception") || e.contains("error"));
        }
    }
}
