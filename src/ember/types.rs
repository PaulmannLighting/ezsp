/// Type alias for the Ember node ID.
pub type NodeId = u16;

/// Type alias for the Ember PAN ID.
pub type PanId = u16;

/// Type alias for the Ember multicast ID.
pub type MulticastId = u16;

/// Type alias for the Eui64.
pub type Eui64 = u64;

/// Type alias for the Ember certificate data.
pub type CertificateData = [u8; 48];

/// Type alias for the Ember public key data.
pub type PublicKeyData = [u8; 22];

/// Type alias for the Ember private key data.
pub type PrivateKeyData = [u8; 21];

/// Type alias for the Ember SMAC data.
pub type SmacData = [u8; 16];

/// Type alias for the Ember signature data.
pub type SignatureData = [u8; 42];

/// Type alias for the Ember certificate 283k1 data.
pub type Certificate283k1Data = [u8; 74];

/// Type alias for the Ember public key 283k1 data.
pub type PublicKey283k1Data = [u8; 37];

/// Type alias for the Ember private key 283k1 data.
pub type PrivateKey283k1Data = [u8; 36];

/// Type alias for the Ember signature 283k1 data.
pub type Signature283k1Data = [u8; 72];

/// Type alias for the Ember message digest.
pub type MessageDigest = [u8; 16];

/// Type alias for the Ember device duty cycles.
pub type DeviceDutyCycles = [u8; 134];
