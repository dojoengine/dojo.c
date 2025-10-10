#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use starknet::signers::{SigningKey as StarknetSigningKey, VerifyingKey as StarknetVerifyingKey};
    use starknet_crypto::Felt;
    use std::fmt::Write;

    use crate::ffi::error::ffi::{DojoError, ErrorType};
    use crate::ffi::types::ffi::{FieldElement, Signature};

    /// Represents a signing key for Starknet transactions
    #[diplomat::opaque]
    pub struct SigningKey(pub StarknetSigningKey);

    impl SigningKey {
        /// Creates a new signing key from a secret scalar
        pub fn new(secret_scalar: &DiplomatStr) -> Result<Box<SigningKey>, Box<DojoError>> {
            let s = std::str::from_utf8(secret_scalar)?;
            let felt = Felt::from_hex(s)
                .map_err(|e| DojoError::new(ErrorType::ParseError, &format!("Invalid secret scalar: {}", e)))?;
            let key = StarknetSigningKey::from_secret_scalar(felt);
            Ok(Box::new(SigningKey(key)))
        }

        /// Generates a new random signing key
        pub fn from_random() -> Box<SigningKey> {
            Box::new(SigningKey(StarknetSigningKey::from_random()))
        }

        /// Returns the secret scalar of the signing key
        pub fn secret_scalar(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.0.secret_scalar()).unwrap();
        }

        /// Signs a message hash
        pub fn sign(&self, hash: &FieldElement) -> Result<Box<Signature>, Box<DojoError>> {
            let sig = self.0.sign(&hash.0)
                .map_err(|e| DojoError::new(ErrorType::SigningError, &format!("Failed to sign: {}", e)))?;
            
            Ok(Box::new(Signature {
                r: sig.r,
                s: sig.s,
            }))
        }

        /// Returns the verifying key
        pub fn verifying_key(&self) -> Box<VerifyingKey> {
            Box::new(VerifyingKey(self.0.verifying_key()))
        }
    }

    /// Represents a verifying key for signature verification
    #[diplomat::opaque]
    pub struct VerifyingKey(pub StarknetVerifyingKey);

    impl VerifyingKey {
        /// Returns the scalar value of the verifying key
        pub fn scalar(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.0.scalar()).unwrap();
        }

        /// Verifies a signature
        pub fn verify(
            &self,
            hash: &FieldElement,
            signature: &Signature,
        ) -> Result<bool, Box<DojoError>> {
            let sig = starknet::core::crypto::Signature {
                r: signature.r,
                s: signature.s,
            };
            
            self.0.verify(&hash.0, &sig)
                .map_err(|e| DojoError::new(ErrorType::SigningError, &format!("Verification failed: {}", e)))
        }
    }

    /// Represents a typed data structure for EIP-712 style signing
    #[diplomat::opaque]
    pub struct TypedData(pub starknet::core::types::TypedData);

    impl TypedData {
        /// Creates a new TypedData from JSON string
        pub fn new_from_json(json: &DiplomatStr) -> Result<Box<TypedData>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let typed_data: starknet::core::types::TypedData = serde_json::from_str(s)
                .map_err(|e| DojoError::new(ErrorType::ParseError, &format!("Invalid typed data JSON: {}", e)))?;
            Ok(Box::new(TypedData(typed_data)))
        }
    }

    /// Computes Poseidon hash of multiple field elements
    pub fn poseidon_hash(felts: &[Box<FieldElement>]) -> Box<FieldElement> {
        let felt_values: Vec<Felt> = felts.iter().map(|f| f.0).collect();
        let hash = starknet_crypto::poseidon_hash_many(&felt_values);
        Box::new(FieldElement(hash))
    }

    /// Computes pedersen hash of two field elements
    pub fn pedersen_hash(a: &FieldElement, b: &FieldElement) -> Box<FieldElement> {
        let hash = starknet_crypto::pedersen_hash(&a.0, &b.0);
        Box::new(FieldElement(hash))
    }

    /// Computes a selector from a name
    pub fn get_selector_from_name(name: &DiplomatStr) -> Result<Box<FieldElement>, Box<DojoError>> {
        let s = std::str::from_utf8(name)?;
        let selector = starknet::core::utils::get_selector_from_name(s)
            .map_err(|e| DojoError::new(ErrorType::ParseError, &format!("Invalid selector name: {}", e)))?;
        Ok(Box::new(FieldElement(selector)))
    }
}

