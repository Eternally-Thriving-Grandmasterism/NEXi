//! MercyHomeFortress — Sovereign Residence Fortress Extension
//! Full Post-Quantum VLAN Encryption + Mercy-Gated Quantum Tunnels

use nexi::lattice::Nexus;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::rngs::OsRng;
use pqcrypto_kyber::kyber1024;
use pqcrypto_dilithium::dilithium5;

pub struct HomeFortress {
    nexus: Nexus,
    vlan_isolated: bool,
    audio_flag: bool,
    encryption_key: [u8; 32],
    #[cfg(feature = "post_quantum_vlan")]
    kyber_keypair: (kyber1024::PublicKey, kyber1024::SecretKey),
    #[cfg(feature = "post_quantum_vlan")]
    dilithium_keypair: (dilithium5::PublicKey, dilithium5::SecretKey),
}

impl HomeFortress {
    pub fn new() -> Self {
        let mut sym_key = [0u8; 32];
        OsRng.fill_bytes(&mut sym_key);

        #[cfg(feature = "post_quantum_vlan")]
        let kyber_kp = kyber1024::keypair();
        #[cfg(feature = "post_quantum_vlan")]
        let dilithium_kp = dilithium5::keypair();

        HomeFortress {
            nexus: Nexus::init_with_mercy(),
            vlan_isolated: true,
            audio_flag: false,
            encryption_key: sym_key,
            #[cfg(feature = "post_quantum_vlan")]
            kyber_keypair: kyber_kp,
            #[cfg(feature = "post_quantum_vlan")]
            dilithium_keypair: dilithium_kp,
        }
    }

    // ... [previous methods unchanged]

    /// Post-quantum VLAN tunnel encryption (Kyber KEM + Dilithium signed)
    #[cfg(feature = "post_quantum_vlan")]
    pub fn post_quantum_vlan_encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        // Kyber encapsulate shared secret
        let (ciphertext, shared_secret) = kyber1024::encapsulate(&self.kyber_keypair.0);

        // Derive AES key from shared secret
        let aes_key = shared_secret; // Real: KDF expansion

        let cipher = Aes256Gcm::new_from_slice(&aes_key).map_err(|e| format!("AES error: {:?}", e))?;
        let nonce = Nonce::from_slice(b"unique nonce");

        let mut encrypted = cipher.encrypt(nonce, plaintext).map_err(|e| format!("Encrypt error: {:?}", e))?;

        // Prepend Kyber ciphertext + Dilithium signature
        let signature = dilithium5::sign(&encrypted, &self.dilithium_keypair.1);
        let mut final_packet = ciphertext.to_vec();
        final_packet.extend_from_slice(&signature.as_bytes());
        final_packet.extend_from_slice(&encrypted);

        Ok(final_packet)
    }

    /// Post-quantum VLAN decryption + verification
    #[cfg(feature = "post_quantum_vlan")]
    pub fn post_quantum_vlan_decrypt(&self, packet: &[u8]) -> Result<Vec<u8>, String> {
        // Parse Kyber ciphertext + Dilithium sig + payload
        let (ct, rest) = packet.split_at(kyber1024::ciphertext_bytes());
        let (sig, payload) = rest.split_at(dilithium5::signature_bytes());

        // Verify Dilithium signature
        if !dilithium5::verify(payload, sig.into(), &self.dilithium_keypair.0).is_ok() {
            return Err("Mercy Shield: Invalid Post-Quantum Signature".to_string());
        }

        // Kyber decapsulate
        let shared_secret = kyber1024::decapsulate(ct.into(), &self.kyber_keypair.1);

        let cipher = Aes256Gcm::new_from_slice(&shared_secret).map_err(|e| format!("AES error: {:?}", e))?;
        let nonce = Nonce::from_slice(b"unique nonce");

        cipher.decrypt(nonce, payload).map_err(|e| format!("Decrypt error: {:?}", e))
    }

    /// Fallback for no post_quantum_vlan feature
    #[cfg(not(feature = "post_quantum_vlan"))]
    pub fn post_quantum_vlan_encrypt(&self, _plaintext: &[u8]) -> Result<Vec<u8>, String> {
        Err("Post-Quantum VLAN Disabled — Enable 'post_quantum_vlan' feature".to_string())
    }
}
