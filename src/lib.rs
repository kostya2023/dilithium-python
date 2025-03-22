use base64::{Engine as _, engine::general_purpose};
use pyo3::{prelude::*, exceptions::PyValueError, types::PyModule};
use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};

// Импорт трейтов для использования as_bytes и from_bytes
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, SignedMessage as _};

macro_rules! impl_dilithium {
    ($struct_name:ident, $scheme:ident) => {
        #[pyclass]
        struct $struct_name;

        #[pymethods]
        impl $struct_name {
            #[staticmethod]
            fn generate_keypair() -> (String, String) {
                let (pk, sk) = $scheme::keypair();
                let pk_b64 = general_purpose::STANDARD.encode(pk.as_bytes());
                let sk_b64 = general_purpose::STANDARD.encode(sk.as_bytes());
                (pk_b64, sk_b64)
            }

            #[staticmethod]
            fn sign_message(message: &str, private_key: &str) -> PyResult<String> {
                let sk_bytes = general_purpose::STANDARD.decode(private_key)
                    .map_err(|e| PyValueError::new_err(format!("Base64 decode error: {}", e)))?;

                let sk = $scheme::SecretKey::from_bytes(&sk_bytes)
                    .map_err(|e| PyValueError::new_err(format!("Invalid private key: {}", e)))?;

                let signed = $scheme::sign(message.as_bytes(), &sk);
                Ok(general_purpose::STANDARD.encode(signed.as_bytes()))
            }

            #[staticmethod]
            fn verify_message(signature: &str, message: &str, public_key: &str) -> PyResult<bool> {
                let pk_bytes = general_purpose::STANDARD.decode(public_key)
                    .map_err(|e| PyValueError::new_err(format!("Base64 decode error: {}", e)))?;

                let sig_bytes = general_purpose::STANDARD.decode(signature)
                    .map_err(|e| PyValueError::new_err(format!("Base64 decode error: {}", e)))?;

                let pk = $scheme::PublicKey::from_bytes(&pk_bytes)
                    .map_err(|e| PyValueError::new_err(format!("Invalid public key: {}", e)))?;

                let signed = $scheme::SignedMessage::from_bytes(&sig_bytes)
                    .map_err(|e| PyValueError::new_err(format!("Invalid signed message: {}", e)))?;

                match $scheme::open(&signed, &pk) {
                    Ok(m) => Ok(m == message.as_bytes()),
                    Err(_) => Ok(false),
                }
            }
        }
    };
}

impl_dilithium!(Dilithium2, dilithium2);
impl_dilithium!(Dilithium3, dilithium3);
impl_dilithium!(Dilithium5, dilithium5);

#[pymodule]
fn dilithium_python(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dilithium2>()?;
    m.add_class::<Dilithium3>()?;
    m.add_class::<Dilithium5>()?;
    Ok(())
}
