use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::result;

/// Represents the type of the layer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LayerType(u8);

impl Display for LayerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LayerTypes::Ethernet => "Ethernet",
                LayerTypes::Arp => "ARP",
                LayerTypes::Ipv4 => "IPv4",
                LayerTypes::Tcp => "TCP",
                LayerTypes::Udp => "UDP",
                _ => "unknown",
            }
        )
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod LayerTypes {
    use super::LayerType;

    // Ethernet
    pub const Ethernet: LayerType = LayerType(0);
    // ARP
    pub const Arp: LayerType = LayerType(1);
    // IPv4
    pub const Ipv4: LayerType = LayerType(2);
    // TCP
    pub const Tcp: LayerType = LayerType(3);
    // UDP
    pub const Udp: LayerType = LayerType(4);
}

/// Represents an error when serialize layers.
#[derive(Debug)]
pub enum SerializeError {
    BufferTooSmallError,
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            SerializeError::BufferTooSmallError => write!(f, "buffer too small"),
        }
    }
}

impl Error for SerializeError {}

pub type SerializeResult = result::Result<usize, SerializeError>;

/// Represents a layer.
pub trait Layer: Display {
    // Get the type of the `Layer`.
    fn get_type(&self) -> LayerType;

    // Get The size of the `Layer` when converted into a byte-array.
    fn get_size(&self) -> usize;

    // Serialize the `Layer` into a byte-array.
    fn serialize(&self, buffer: &mut [u8]) -> SerializeResult;

    // Recalculate the length and serialize the `Layer` into a byte-array.
    fn serialize_n(&self, buffer: &mut [u8], n: usize) -> SerializeResult;
}

use super::arp;
use super::ethernet;
use super::ipv4;
use super::tcp;
use super::udp;

#[derive(Debug, Clone)]
pub enum Layers {
    Ethernet(ethernet::Ethernet),
    Arp(arp::Arp),
    Ipv4(ipv4::Ipv4),
    Tcp(tcp::Tcp),
    Udp(udp::Udp),
}

impl Display for Layers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Layers::Ethernet(ref layer) => layer.fmt(f),
            Layers::Arp(ref layer) => layer.fmt(f),
            Layers::Ipv4(ref layer) => layer.fmt(f),
            Layers::Tcp(ref layer) => layer.fmt(f),
            Layers::Udp(ref layer) => layer.fmt(f),
        }
    }
}

impl Layer for Layers {
    fn get_type(&self) -> LayerType {
        match self {
            Layers::Ethernet(ref layer) => layer.get_type(),
            Layers::Arp(ref layer) => layer.get_type(),
            Layers::Ipv4(ref layer) => layer.get_type(),
            Layers::Tcp(ref layer) => layer.get_type(),
            Layers::Udp(ref layer) => layer.get_type(),
        }
    }

    fn get_size(&self) -> usize {
        match self {
            Layers::Ethernet(ref layer) => layer.get_size(),
            Layers::Arp(ref layer) => layer.get_size(),
            Layers::Ipv4(ref layer) => layer.get_size(),
            Layers::Tcp(ref layer) => layer.get_size(),
            Layers::Udp(ref layer) => layer.get_size(),
        }
    }

    fn serialize(&self, buffer: &mut [u8]) -> SerializeResult {
        match self {
            Layers::Ethernet(ref layer) => layer.serialize(buffer),
            Layers::Arp(ref layer) => layer.serialize(buffer),
            Layers::Ipv4(ref layer) => layer.serialize(buffer),
            Layers::Tcp(ref layer) => layer.serialize(buffer),
            Layers::Udp(ref layer) => layer.serialize(buffer),
        }
    }

    fn serialize_n(&self, buffer: &mut [u8], n: usize) -> SerializeResult {
        match self {
            Layers::Ethernet(ref layer) => layer.serialize_n(buffer, n),
            Layers::Arp(ref layer) => layer.serialize_n(buffer, n),
            Layers::Ipv4(ref layer) => layer.serialize_n(buffer, n),
            Layers::Tcp(ref layer) => layer.serialize_n(buffer, n),
            Layers::Udp(ref layer) => layer.serialize_n(buffer, n),
        }
    }
}
