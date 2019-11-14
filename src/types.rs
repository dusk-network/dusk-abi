use core::mem;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn zero() -> Self {
        H256(Default::default())
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for H256 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl core::fmt::Debug for H256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Digest(")?;
        for i in 0..32 {
            write!(f, "{:02x}", self.0[i])?;
        }
        write!(f, ")")
    }
}

const SIGNATURE_BYTES: usize = 64;

/// Compability signature type, with no impls
#[repr(C)]
pub struct Signature([u8; SIGNATURE_BYTES]);

impl Signature {
    pub fn as_array(&self) -> &[u8; 64] {
        &self.0
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

// Serde hack.
//
// Due to rust not yet having stable const genecics, serde is not able to automatically derive
// de/serialization for arrays larger than 32.
//
// In order not to have to manually implement Serialize,
// we create a wrapper type with the same memory layout,
// that can still be automatically derived.
#[repr(C)]
#[derive(Serialize, Deserialize)]
struct SignatureSerializationHack([u8; 32], [u8; 32]);

impl core::fmt::Debug for Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Signature(")?;
        for i in 0..64 {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}

impl Signature {
    pub fn new() -> Self {
        Signature([42u8; SIGNATURE_BYTES])
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut buf = [0u8; 64];
        buf.copy_from_slice(slice);
        Signature(buf)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unsafe {
            let hack: &SignatureSerializationHack = mem::transmute(self);

            // make sure this serialization hack is sound
            // TODO: verify that this is optimised out on non-debug builds
            {
                let mut buf = [0u8; 64];
                fermion::encode(hack, &mut buf).expect("Cannot fail");

                for i in 0..64 {
                    debug_assert!(buf[i] == self.0[i])
                }
            }

            hack.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hack = SignatureSerializationHack::deserialize(deserializer)?;
        unsafe {
            let _self = mem::transmute(hack);
            Ok(_self)
        }
    }
}

#[cfg(feature = "std")]
mod content {
    use kelvin::{ByteHash, Content, Source};

    use super::H256;
    use std::io::{self, Write};

    impl<H: ByteHash> Content<H> for H256 {
        fn persist(&mut self, sink: &mut dyn Write) -> io::Result<()> {
            unimplemented!()
        }

        fn restore(source: &mut Source<H>) -> io::Result<Self> {
            unimplemented!()
        }
    }

}
