use bytes::BufMut;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    num::{NonZeroU16, NonZeroU8},
};

pub trait Descriptor {
    /// Identifies the descriptor.
    const TYPE: u8;

    /// Returns the length of the descriptor in bytes when encoded.
    fn length(&self) -> NonZeroU8;

    /// Encodes the descriptor into the buffer.
    fn encode(&self, buf: &mut impl BufMut);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ReleaseNumber {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
}

impl ReleaseNumber {
    #[inline]
    #[must_use]
    pub const fn bcd(&self) -> u16 {
        ((self.major as u16) << 8) | ((self.minor as u16) << 4) | (self.subminor as u16)
    }
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

impl From<EndpointZeroMaximumPacketSize> for u8 {
    fn from(size: EndpointZeroMaximumPacketSize) -> Self {
        size.0
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

impl Descriptor for Device {
    const TYPE: u8 = 1;

    #[inline]
    #[must_use]
    fn length(&self) -> NonZeroU8 {
        unsafe { NonZeroU8::new_unchecked(18) }
    }

    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.length().get());
        buf.put_u8(Self::TYPE);
        buf.put_u16(self.usb_release.bcd());

        if let Some(functionality) = self.functionality {
            buf.put_u8(functionality.class);
            buf.put_u8(functionality.sub_class);
            buf.put_u8(functionality.protocol);
        } else {
            buf.put_u8(0);
            buf.put_u8(0);
            buf.put_u8(0);
        }

        buf.put_u8(u8::from(self.maximum_packet_size));
        buf.put_u16(self.vendor);
        buf.put_u16(self.product);
        buf.put_u16(self.device_release.bcd());
        buf.put_u8(self.imanufacturer.map_or(0, NonZeroU8::get));
        buf.put_u8(self.iproduct.map_or(0, NonZeroU8::get));
        buf.put_u8(self.iserial.map_or(0, NonZeroU8::get));
        buf.put_u8(self.nconfigurations.get());
    }
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

impl Descriptor for Configuration {
    const TYPE: u8 = 2;

    #[inline]
    #[must_use]
    fn length(&self) -> NonZeroU8 {
        unsafe { NonZeroU8::new_unchecked(9) }
    }

    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.length().get());
        buf.put_u8(Self::TYPE);
        buf.put_u16(self.total_length.get());
        buf.put_u8(self.ninterfaces.get());
        buf.put_u8(self.value);
        buf.put_u8(self.iconfiguration.map_or(0, NonZeroU8::get));

        let mut attributes = 0;

        if self.is_bus_powered {
            attributes |= 1 << 7;
        }

        if self.is_self_powered {
            attributes |= 1 << 6;
        }

        if self.is_remote_wakeup_supported {
            attributes |= 1 << 5;
        }

        buf.put_u8(attributes);
        buf.put_u8(self.maximum_power);
    }
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

impl Descriptor for Interface {
    const TYPE: u8 = 4;

    #[inline]
    #[must_use]
    fn length(&self) -> NonZeroU8 {
        unsafe { NonZeroU8::new_unchecked(9) }
    }

    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.length().get());
        buf.put_u8(Self::TYPE);
        buf.put_u8(self.number);
        buf.put_u8(self.alternate_setting);
        buf.put_u8(self.nendpoints);

        if let Some(functionality) = self.functionality {
            buf.put_u8(functionality.class);
            buf.put_u8(functionality.sub_class);
            buf.put_u8(functionality.protocol);
        } else {
            buf.put_u8(0);
            buf.put_u8(0);
            buf.put_u8(0);
        }

        buf.put_u8(self.index.map_or(0, NonZeroU8::get));
    }
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

impl Descriptor for Endpoint {
    const TYPE: u8 = 5;

    #[inline]
    #[must_use]
    fn length(&self) -> NonZeroU8 {
        unsafe { NonZeroU8::new_unchecked(7) }
    }

    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.length().get());
        buf.put_u8(Self::TYPE);
        buf.put_u8(self.number);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringHeader {
    /// Defines the set of supported languages.
    pub languages: HashSet<u16>,
}

impl Descriptor for StringHeader {
    const TYPE: u8 = 3;

    #[inline]
    #[must_use]
    fn length(&self) -> NonZeroU8 {
        unsafe {
            NonZeroU8::new_unchecked(
                u8::try_from(2 + (2 * self.languages.len())).unwrap_or_else(|_| unreachable!()),
            )
        }
    }

    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.length().get());
        buf.put_u8(Self::TYPE);

        for language in self.languages.iter().copied() {
            buf.put_u16(language);
        }
    }
}
