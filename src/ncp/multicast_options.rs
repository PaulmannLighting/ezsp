/// Multicast delivery options for [`Ncp::multicast`](crate::Ncp::multicast).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MulticastOptions {
    hops: u8,
    nonmember_radius: u8,
}

impl MulticastOptions {
    /// Creates multicast delivery options.
    #[must_use]
    pub const fn new(hops: u8, nonmember_radius: u8) -> Self {
        Self {
            hops,
            nonmember_radius,
        }
    }

    /// Returns the number of hops for member-node forwarding.
    #[must_use]
    pub const fn hops(self) -> u8 {
        self.hops
    }

    /// Returns the nonmember forwarding radius.
    #[must_use]
    pub const fn nonmember_radius(self) -> u8 {
        self.nonmember_radius
    }
}
