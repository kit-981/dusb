use crate::language::Language;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    num::{NonZeroU16, NonZeroU8},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ReleaseNumber {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct InvalidValueError;

impl Display for InvalidValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid value")
    }
}

impl Error for InvalidValueError {}

/// A safe wrapper around the packet size for endpoint zero.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct EndpointZeroMaximumPacketSize(u8);

impl EndpointZeroMaximumPacketSize {
    /// # Errors
    ///
    /// An error is returned if `size` is invalid.
    pub fn new(size: u8) -> Result<Self, InvalidValueError> {
        match size {
            8 | 16 | 32 | 64 => Ok(Self(size)),
            _ => Err(InvalidValueError),
        }
    }

    /// # Safety
    ///
    /// Instantiation is infallible because the value is not checked for validity. Logic interacting
    /// with an instance of [`Self`] may assume that the underlying value is valid such that
    /// improper use of this function may cause unexpected behaviour. For this reason, use of this
    /// function is unsafe.
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked(size: u8) -> Self {
        Self(size)
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FunctionalityIdentifier {
    pub class: u8,
    pub sub_class: u8,
    pub protocol: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Device {
    /// Identifies the release of the USB specification that the device and its descriptors are
    /// compliant with.
    pub usb_release: ReleaseNumber,
    /// Identifies the device's functionality.
    pub functionality: Option<FunctionalityIdentifier>,
    /// Specifies the maximum packet size for endpoint zero.
    pub maximum_packet_size: EndpointZeroMaximumPacketSize,
    /// Identifies the vendor.
    pub vendor: u16,
    /// Identifies the product.
    pub product: u16,
    /// Specifies the device's release number.
    pub device_release: ReleaseNumber,
    /// Specifies the index of the string descriptor that describes the manufacturer.
    pub imanufacturer: Option<NonZeroU8>,
    /// Specifies the index of the string descriptor that describes the product.
    pub iproduct: Option<NonZeroU8>,
    /// Specifies the index of the string descriptor that describes the device's serial number.
    pub iserial: Option<NonZeroU8>,
    /// Specifies the number of possible configurations.
    pub nconfigurations: NonZeroU8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Configuration {
    /// Specifies the total length of the configuration. This length is the combined length of the
    /// associated configuration, interface, endpoint, class, and vendor descriptors.
    pub total_length: NonZeroU16,
    /// Specifies the number of interfaces.
    pub ninterfaces: NonZeroU8,
    /// Identifies the configuration.
    pub value: u8,
    /// Specifies the index of the string descriptor describing the configuration.
    pub iconfiguration: Option<NonZeroU8>,
    /// Specifies whether or not the device is bus-powered.
    pub is_bus_powered: bool,
    /// Specifies whether or not the device is self-powered.
    pub is_self_powered: bool,
    /// Specifies whether or not remote wakeup is supported.
    pub is_remote_wakeup_supported: bool,
    /// Specifies the maximum power consumption of the device. The value is expressed in units of 2
    /// mA.
    pub maximum_power: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Interface {
    /// Identifies the interface.
    pub number: u8,
    /// Identifies the alternate setting.
    pub alternate_setting: u8,
    /// Specifies the number of endpoints used by the interface excluding endpoint zero.
    pub nendpoints: u8,
    /// Identifies the interface's functionality.
    pub functionality: Option<FunctionalityIdentifier>,
    /// Specifies the index of the string descriptor that describes the interface.
    pub index: Option<NonZeroU8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EndpointDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ControlEndpoint {
    /// Specifies the maximum packet size that the endpoint is capable of sending or receiving.
    pub maximum_packet_size: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct IsochronousEndpoint {
    /// Specifies the direction.
    pub direction: EndpointDirection,
    /// Specifies the maximum packet size that the endpoint is capable of sending or receiving.
    pub maximum_packet_size: u8,
}

impl IsochronousEndpoint {
    pub const INTERVAL: u8 = 1;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BulkEndpoint {
    /// Specifies the direction.
    pub direction: EndpointDirection,
    /// Specifies the maximum packet size that the endpoint is capable of sending or receiving.
    pub maximum_packet_size: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct InterruptEndpoint {
    /// Specifies the direction.
    pub direction: EndpointDirection,
    /// Specifies the maximum packet size that the endpoint is capable of sending or receiving.
    pub maximum_packet_size: u8,
    /// Defines the interval for polling endpoint data transfers expressed in milliseconds.
    pub interval: NonZeroU8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EndpointKind {
    Control(ControlEndpoint),
    Isochronous(IsochronousEndpoint),
    Bulk(BulkEndpoint),
    Interrupt(InterruptEndpoint),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Endpoint {
    /// Identifies the endpoint.
    pub number: u8,
    pub kind: EndpointKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StringHeaderLanguage {
    Known(Language),
    Unknown(u16),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringHeader {
    /// Defines the set of supported languages.
    pub languages: HashSet<StringHeaderLanguage>,
}
