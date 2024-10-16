/// 16-bit `ZigBee` network address.
pub type NodeId = u16;

/// 802.15.4 PAN ID.
pub type PanId = u16;

/// 16-bit `ZigBee` multicast group identifier.
pub type MulticastId = u16;

/// EUI 64-bit ID (an IEEE address).
pub type Eui64 = macaddr::MacAddr8;

/// The implicit certificate used in `CBKE`.
pub type CertificateData = [u8; 48];

/// The public key data used in `CBKE`.
pub type PublicKeyData = [u8; 22];

/// The private key data used in `CBKE`.
pub type PrivateKeyData = [u8; 21];

/// The Shared Message Authentication Code data used in `CBKE`.
pub type SmacData = [u8; 16];

/// An ECDSA signature
pub type SignatureData = [u8; 42];

/// The implicit certificate used in `CBKE`.
pub type Certificate283k1Data = [u8; 74];

/// The public key data used in `CBKE`.
pub type PublicKey283k1Data = [u8; 37];

/// The private key data used in `CBKE`.
pub type PrivateKey283k1Data = [u8; 36];

/// An ECDSA signature
pub type Signature283k1Data = [u8; 72];

/// The calculated digest of a message
pub type MessageDigest = [u8; 16];

/// Consumed duty cycles up to maxDevices.
///
/// When the number of children that are being monitored is less than maxDevices,
/// the `EmberNodeId` element in the `EmberPerDeviceDutyCycle` will be `0xFFFF`.
pub type DeviceDutyCycles = [u8; 134];
