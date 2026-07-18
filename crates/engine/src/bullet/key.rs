use slotmap::new_key_type;

new_key_type! {
    pub struct BulletKey;
}

impl BulletKey {
    pub fn as_ffi(self) -> u64 {
        self.0.as_ffi()
    }

    pub fn from_ffi(ffi: u64) -> Self {
        Self(slotmap::KeyData::from_ffi(ffi))
    }
}
