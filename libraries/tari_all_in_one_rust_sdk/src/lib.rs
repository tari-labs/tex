pub type MicroMinotari = u64;

pub mod indexer;
pub mod wallet_daemon;

pub mod encrypted_data {
    // Copyright 2022 The Tari Project
    //
    // Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
    // following conditions are met:
    //
    // 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
    // disclaimer.
    //
    // 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
    // following disclaimer in the documentation and/or other materials provided with the distribution.
    //
    // 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
    // products derived from this software without specific prior written permission.
    //
    // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
    // INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
    // SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
    // WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
    // USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE
    //
    // Portions of this file were originally copyrighted (c) 2018 The Grin Developers, issued under the Apache License,
    // Version 2.0, available at http://www.apache.org/licenses/LICENSE-2.0.

    //! Encrypted data using the extended-nonce variant XChaCha20-Poly1305 encryption with secure random nonce.

    use std::mem::size_of;

    use blake2::Blake2b;
    use chacha20poly1305::{
        Key, KeyInit, Tag, XChaCha20Poly1305, XNonce, aead::AeadInPlace, aead::Error as AeadError,
    };
    use derive_more::derive::{Display, Error, From};
    use digest::{FixedOutput, consts::U32, generic_array::GenericArray};
    use tari_crypto::{
        hash_domain,
        hashing::DomainSeparatedHasher,
        ristretto::{RistrettoSecretKey, pedersen::PedersenCommitment},
    };
    use tari_utilities::{ByteArray, ByteArrayError as InnerByteArrayError};
    use zeroize::Zeroizing;

    use crate::MicroMinotari;

    // Useful size constants, each in bytes
    const SIZE_NONCE: usize = size_of::<XNonce>();
    const SIZE_VALUE: usize = size_of::<u64>();
    const SIZE_MASK: usize = size_of::<Key>();
    const SIZE_TAG: usize = size_of::<Tag>();
    /// AEAD associated data
    const ENCRYPTED_DATA_AAD: &[u8] = b"TARI_AAD_VALUE_AND_MASK_EXTEND_NONCE_VARIANT";

    hash_domain!(
        TransactionSecureNonceKdfDomain,
        "com.tari.base_layer.core.transactions.secure_nonce_kdf",
        0
    );

    /// EncryptedOpenings errors
    #[derive(Debug, Error, Display, From)]
    pub enum EncryptedDataError {
        #[display("Encryption failed: {source}")]
        EncryptionFailed { source: AeadError },
        #[display("Incorrect length: {source}")]
        IncorrectLength { source: ByteArrayError },
    }

    #[derive(Debug, Display, From)]
    pub struct ByteArrayError(InnerByteArrayError);

    impl std::error::Error for ByteArrayError {}

    /// Authenticate and decrypt the value and mask
    /// Note: This design (similar to other AEADs) is not key committing, thus the caller must not rely on successful
    ///       decryption to assert that the expected key was used
    pub fn decrypt_data(
        encryption_key: &RistrettoSecretKey,
        commitment: &PedersenCommitment,
        encrypted_data: &[u8],
    ) -> Result<(MicroMinotari, RistrettoSecretKey), EncryptedDataError> {
        let tag = Tag::from_slice(&{ encrypted_data }[..SIZE_TAG]);
        let nonce = XNonce::from_slice(&{ encrypted_data }[SIZE_TAG..SIZE_TAG + SIZE_NONCE]);
        let mut bytes = Zeroizing::new(vec![
            0;
            encrypted_data
                .len()
                .saturating_sub(SIZE_TAG)
                .saturating_sub(SIZE_NONCE)
        ]);
        bytes.clone_from_slice(&encrypted_data[SIZE_TAG + SIZE_NONCE..]);
        let mut aead_key = Key::default();
        DomainSeparatedHasher::<Blake2b<U32>, TransactionSecureNonceKdfDomain>::new_with_label(
            "encrypted_value_and_mask",
        )
        .chain(encryption_key.as_bytes())
        .chain(commitment.as_bytes())
        .finalize_into(GenericArray::from_mut_slice(&mut aead_key));
        let cipher = XChaCha20Poly1305::new(GenericArray::from_slice(&aead_key));
        cipher.decrypt_in_place_detached(nonce, ENCRYPTED_DATA_AAD, bytes.as_mut_slice(), tag)?;
        let mut value_bytes = [0u8; SIZE_VALUE];
        value_bytes.clone_from_slice(&bytes[0..SIZE_VALUE]);
        Ok((
            u64::from_le_bytes(value_bytes),
            RistrettoSecretKey::from_canonical_bytes(&bytes[SIZE_VALUE..SIZE_VALUE + SIZE_MASK])
                .map_err(ByteArrayError::from)?,
        ))
    }
}
