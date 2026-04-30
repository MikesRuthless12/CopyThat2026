//! Phase 44.4c — TCG OPAL Storage Specification packet-encoding
//! scaffold (gated behind the `experimental-tcg-opal` feature).
//!
//! This module ships the byte-marshaling code for the OPAL command
//! envelopes — `ComPacket`, `Packet`, and `SubPacket` — plus the
//! token-streaming primitives from TCG SWG Core Spec §3.2 needed
//! to encode the `RevertSP` and `StartSession` method calls. It
//! does NOT ship the IOCTL / SCSI passthrough transport that
//! would actually deliver these packets to a drive — that lands
//! in a future phase once we have a hardware test bed of
//! Self-Encrypting Drives we can safely RevertSP without
//! risking user data.
//!
//! The module exists now so:
//! 1. The byte layout work — which is non-trivial and easy to
//!    get wrong — can be reviewed and unit-tested independently
//!    of the destructive transport.
//! 2. The Linux helper's existing `sedutil-cli` shell-out path
//!    can be replaced with native packet emission once the
//!    transport lands, without re-litigating the encoding.
//! 3. CI builds the encoding under the feature flag and exercises
//!    its golden-vector tests, so the byte layouts don't drift
//!    silently as the surrounding code evolves.
//!
//! # Threat model boundary
//!
//! The encoder emits canonical OPAL packets. It does NOT emit
//! anything destructive on its own — every function returns
//! `Vec<u8>` or borrowed slices. Callers (a future
//! `OpalTransport` trait) decide whether to ship those bytes to
//! a drive. Until that transport lands, this module is pure data
//! transformation and cannot harm a drive.
//!
//! # References
//!
//! - TCG Storage Architecture Core Specification, Version 2.01,
//!   Revision 1.00 (TCG-SWG-Core)
//! - TCG Storage Security Subsystem Class: Opal, Version 2.02,
//!   Revision 1.00 (TCG-SSC-Opal)
//! - The `ComPacket` / `Packet` / `SubPacket` byte layouts are
//!   defined in TCG-SWG-Core §3.2.3.

#![cfg(feature = "experimental-tcg-opal")]

/// TCG-SWG-Core §3.2.3.1 — `ComPacket` envelope. Wraps zero or
/// more [`Packet`]s when the transport delivers them in a single
/// SCSI Security Protocol payload. 20-byte header.
#[derive(Debug, Clone, Default)]
pub struct ComPacket {
    /// Reserved (4 bytes, must be zero).
    pub reserved: u32,
    /// `ComID` allocated by the SP for this session. Typically the
    /// "base ComID" reported by the drive's Level-0 Discovery
    /// response. Big-endian on the wire.
    pub com_id: u16,
    /// `ComID Extension` — usually zero unless the SP allocated
    /// a multi-channel session. Big-endian on the wire.
    pub com_id_ext: u16,
    /// Total length of the [`Packet`] payload that follows. The
    /// transport overrides this field after marshaling the packets
    /// (see [`ComPacket::encode_into`]). Big-endian on the wire.
    pub outstanding_data: u32,
    /// `MinTransfer` — minimum transfer size before this ComPacket
    /// can be considered "drained." For most non-streaming
    /// commands this is zero. Big-endian on the wire.
    pub min_transfer: u32,
    /// `Length` — payload length in bytes. Big-endian on the wire.
    pub length: u32,
}

impl ComPacket {
    /// Header length on the wire, in bytes.
    pub const HEADER_LEN: usize = 20;

    /// Marshal the 20-byte header into `out`. The caller appends
    /// the [`Packet`] payload after the header; this function does
    /// not own the payload.
    pub fn encode_header_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.reserved.to_be_bytes());
        out.extend_from_slice(&self.com_id.to_be_bytes());
        out.extend_from_slice(&self.com_id_ext.to_be_bytes());
        out.extend_from_slice(&self.outstanding_data.to_be_bytes());
        out.extend_from_slice(&self.min_transfer.to_be_bytes());
        out.extend_from_slice(&self.length.to_be_bytes());
    }
}

/// TCG-SWG-Core §3.2.3.2 — `Packet`. Carries one session's worth
/// of [`SubPacket`]s. 24-byte header.
#[derive(Debug, Clone, Default)]
pub struct Packet {
    /// `TPerSessionNumber` — drive-side session id, returned by
    /// `StartSession`. Zero before session establishment.
    /// Big-endian on the wire.
    pub tper_session: u32,
    /// `HostSessionNumber` — host-side session id. The host picks
    /// this; the drive echoes it. Big-endian on the wire.
    pub host_session: u32,
    /// `SeqNumber` — monotonic per-session sequence number.
    /// Big-endian on the wire.
    pub seq_number: u32,
    /// Reserved (2 bytes, must be zero).
    pub reserved0: u16,
    /// `AckType` — usually zero for non-acknowledged packets.
    pub ack_type: u16,
    /// `Acknowledgement` — sequence number being acknowledged, if
    /// `ack_type` is non-zero. Big-endian on the wire.
    pub ack: u32,
    /// `Length` — payload length in bytes. Big-endian on the wire.
    pub length: u32,
}

impl Packet {
    /// Header length on the wire, in bytes.
    pub const HEADER_LEN: usize = 24;

    /// Marshal the 24-byte header into `out`. The caller appends
    /// the [`SubPacket`] payload after the header.
    pub fn encode_header_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.tper_session.to_be_bytes());
        out.extend_from_slice(&self.host_session.to_be_bytes());
        out.extend_from_slice(&self.seq_number.to_be_bytes());
        out.extend_from_slice(&self.reserved0.to_be_bytes());
        out.extend_from_slice(&self.ack_type.to_be_bytes());
        out.extend_from_slice(&self.ack.to_be_bytes());
        out.extend_from_slice(&self.length.to_be_bytes());
    }
}

/// TCG-SWG-Core §3.2.3.3 — `SubPacket`. Carries the actual token
/// stream (method calls, arguments, results). 12-byte header.
#[derive(Debug, Clone, Default)]
pub struct SubPacket {
    /// Reserved (6 bytes, must be zero).
    pub reserved: [u8; 6],
    /// `Kind` — `0x0000` = data sub-packet (only kind we emit).
    /// Big-endian on the wire.
    pub kind: u16,
    /// `Length` — payload length in bytes. Big-endian on the wire.
    pub length: u32,
}

impl SubPacket {
    /// Header length on the wire, in bytes.
    pub const HEADER_LEN: usize = 12;

    /// `Kind` value for a data sub-packet (the only kind this
    /// scaffold emits).
    pub const KIND_DATA: u16 = 0x0000;

    /// Marshal the 12-byte header into `out`. After the header
    /// comes the token stream, then 4-byte alignment padding.
    pub fn encode_header_into(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.reserved);
        out.extend_from_slice(&self.kind.to_be_bytes());
        out.extend_from_slice(&self.length.to_be_bytes());
    }
}

/// TCG-SWG-Core §3.2.2 — token-stream encoding primitives.
///
/// OPAL methods are streams of self-describing tokens. Each token
/// starts with a tag byte that identifies the type and length:
///
/// | Tag prefix       | Meaning                                |
/// |------------------|----------------------------------------|
/// | `0x00..=0x3F`    | Tiny atom (6-bit signed/unsigned int)  |
/// | `0x80..=0xBF`    | Short atom (length 1-15 bytes)         |
/// | `0xC0..=0xDF`    | Medium atom (length 16-2047 bytes)     |
/// | `0xE0`           | Long atom (length up to 16 MiB)        |
/// | `0xF0`           | Start List token                       |
/// | `0xF1`           | End List token                         |
/// | `0xF2`           | Start Name token                       |
/// | `0xF3`           | End Name token                         |
/// | `0xF8`           | Call token                             |
/// | `0xF9`           | End-of-Data token                      |
/// | `0xFA..=0xFC`    | Status / End-of-Session tokens         |
pub mod tokens {
    /// Start List `[`.
    pub const START_LIST: u8 = 0xF0;
    /// End List `]`.
    pub const END_LIST: u8 = 0xF1;
    /// Start Name (key/value pair start).
    pub const START_NAME: u8 = 0xF2;
    /// End Name.
    pub const END_NAME: u8 = 0xF3;
    /// Call (method invocation).
    pub const CALL: u8 = 0xF8;
    /// End-of-Data (close a method call).
    pub const END_OF_DATA: u8 = 0xF9;
    /// End-of-Session (close a session).
    pub const END_OF_SESSION: u8 = 0xFA;

    /// Encode an unsigned tiny atom (0..=63). Returns the single
    /// tag byte.
    pub fn tiny_uint(value: u8) -> u8 {
        debug_assert!(value <= 0x3F, "tiny atom out of range");
        value & 0x3F
    }

    /// Encode a byte-string short atom (length 1..=15). Pushes the
    /// tag + bytes onto `out`.
    pub fn short_bytes(bytes: &[u8], out: &mut Vec<u8>) {
        debug_assert!(
            (1..=15).contains(&bytes.len()),
            "short atom length out of range — use medium_bytes for longer payloads"
        );
        // Short-atom tag: 0b10x_xxxx where bit 5 = byte/cont, bit
        // 4 = sign (unused for byte strings), bits 0-3 = length.
        // 0xA0 = byte string, unsigned. Length nibble in low 4
        // bits.
        let tag = 0xA0 | (bytes.len() as u8);
        out.push(tag);
        out.extend_from_slice(bytes);
    }

    /// Encode a byte-string medium atom (length 16..=2047). Pushes
    /// the 2-byte tag + bytes onto `out`.
    pub fn medium_bytes(bytes: &[u8], out: &mut Vec<u8>) {
        debug_assert!(
            (16..=2047).contains(&bytes.len()),
            "medium atom length out of range"
        );
        // Medium-atom tag: 0b110x_xxxx_xxxx_xxxx. High byte
        // 0xD0 = byte string, low 11 bits = length, big-endian.
        let len = bytes.len() as u16;
        out.push(0xD0 | ((len >> 8) as u8 & 0x07));
        out.push((len & 0xFF) as u8);
        out.extend_from_slice(bytes);
    }
}

/// TCG-SSC-Opal §3.1 — well-known UIDs used by the methods this
/// scaffold encodes. UIDs are 8-byte big-endian identifiers.
pub mod uids {
    /// `Session Manager` UID — invocation target for
    /// `StartSession`.
    pub const SESSION_MANAGER: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];
    /// `StartSession` method UID.
    pub const START_SESSION: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x02];
    /// `Admin SP` UID — target of `RevertSP`.
    pub const ADMIN_SP: [u8; 8] = [0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x01];
    /// `Locking SP` UID — used as the SP argument to `RevertSP`
    /// when reverting locking-only state.
    pub const LOCKING_SP: [u8; 8] = [0x00, 0x00, 0x02, 0x05, 0x00, 0x00, 0x00, 0x02];
    /// `RevertSP` method UID.
    pub const REVERT_SP: [u8; 8] = [0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x02, 0x02];
    /// `SID` (Anybody) authority UID — used for a PSID-authenticated
    /// `RevertSP`.
    pub const SID_AUTHORITY: [u8; 8] = [0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x06];
    /// `PSID` authority UID — the authority object that authorises
    /// a factory `RevertSP` when paired with the drive's PSID.
    pub const PSID_AUTHORITY: [u8; 8] = [0x00, 0x00, 0x00, 0x09, 0x00, 0x01, 0xFF, 0x01];
}

/// TCG-SSC-Opal §5.3 — encode a `RevertSP` method call against the
/// [`uids::ADMIN_SP`]. The encoded token stream is the SubPacket
/// payload; the caller wraps it in [`SubPacket`] / [`Packet`] /
/// [`ComPacket`] before handing it to a transport.
///
/// The `keep_global_range_key` flag, if true, sets the
/// `KeepGlobalRangeKey` argument so the drive preserves the
/// existing media-encryption key after reverting (rare; used for
/// fleet provisioning).
pub fn encode_revert_sp_call(keep_global_range_key: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.push(tokens::CALL);
    tokens::short_bytes(&uids::ADMIN_SP, &mut out);
    tokens::short_bytes(&uids::REVERT_SP, &mut out);
    out.push(tokens::START_LIST);
    if keep_global_range_key {
        // [ StartName "KeepGlobalRangeKey" 1 EndName ]
        out.push(tokens::START_NAME);
        // The argument name is encoded as a tiny uint (named-arg
        // index 0 per the SSC's per-method numbering for
        // RevertSP). The TCG spec uses uint-tag-name pairs for
        // named arguments; SSC §5.3 lists `KeepGlobalRangeKey`
        // as named-argument 0x06.
        out.push(tokens::tiny_uint(0x06));
        out.push(tokens::tiny_uint(1));
        out.push(tokens::END_NAME);
    }
    out.push(tokens::END_LIST);
    out.push(tokens::END_OF_DATA);
    // Status list: [ 0 0 0 ] — three zeros for the three status
    // fields the drive expects to ack.
    out.push(tokens::START_LIST);
    out.push(tokens::tiny_uint(0));
    out.push(tokens::tiny_uint(0));
    out.push(tokens::tiny_uint(0));
    out.push(tokens::END_LIST);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compacket_header_length_is_20() {
        let cp = ComPacket {
            com_id: 0x07FE,
            length: 100,
            ..ComPacket::default()
        };
        let mut buf = Vec::new();
        cp.encode_header_into(&mut buf);
        assert_eq!(buf.len(), ComPacket::HEADER_LEN);
        // ComID big-endian at offset 4.
        assert_eq!(&buf[4..6], &[0x07, 0xFE]);
        // Length big-endian at offset 16.
        assert_eq!(&buf[16..20], &[0, 0, 0, 100]);
    }

    #[test]
    fn packet_header_length_is_24() {
        let p = Packet {
            host_session: 0xCAFE_BABE,
            seq_number: 1,
            length: 50,
            ..Packet::default()
        };
        let mut buf = Vec::new();
        p.encode_header_into(&mut buf);
        assert_eq!(buf.len(), Packet::HEADER_LEN);
        // HostSessionNumber big-endian at offset 4.
        assert_eq!(&buf[4..8], &[0xCA, 0xFE, 0xBA, 0xBE]);
        // Length big-endian at offset 20.
        assert_eq!(&buf[20..24], &[0, 0, 0, 50]);
    }

    #[test]
    fn subpacket_header_length_is_12() {
        let sp = SubPacket {
            kind: SubPacket::KIND_DATA,
            length: 42,
            ..SubPacket::default()
        };
        let mut buf = Vec::new();
        sp.encode_header_into(&mut buf);
        assert_eq!(buf.len(), SubPacket::HEADER_LEN);
        // Kind big-endian at offset 6.
        assert_eq!(&buf[6..8], &[0x00, 0x00]);
        // Length big-endian at offset 8.
        assert_eq!(&buf[8..12], &[0, 0, 0, 42]);
    }

    #[test]
    fn tiny_uint_encodes_small_values() {
        assert_eq!(tokens::tiny_uint(0), 0x00);
        assert_eq!(tokens::tiny_uint(1), 0x01);
        assert_eq!(tokens::tiny_uint(0x3F), 0x3F);
    }

    #[test]
    fn short_bytes_emits_correct_tag_for_uid() {
        let mut buf = Vec::new();
        tokens::short_bytes(&uids::ADMIN_SP, &mut buf);
        // Tag = 0xA0 | 8 = 0xA8 (byte-string short atom, length 8).
        assert_eq!(buf[0], 0xA8);
        assert_eq!(&buf[1..9], &uids::ADMIN_SP);
    }

    #[test]
    fn revert_sp_call_starts_with_call_token_and_admin_sp_uid() {
        let buf = encode_revert_sp_call(false);
        // Byte 0: CALL token.
        assert_eq!(buf[0], tokens::CALL);
        // Bytes 1..10: short-atom-tagged AdminSP UID.
        assert_eq!(buf[1], 0xA8);
        assert_eq!(&buf[2..10], &uids::ADMIN_SP);
        // Bytes 10..19: short-atom-tagged RevertSP UID.
        assert_eq!(buf[10], 0xA8);
        assert_eq!(&buf[11..19], &uids::REVERT_SP);
        // Byte 19: StartList for the empty argument list.
        assert_eq!(buf[19], tokens::START_LIST);
        // Byte 20: EndList.
        assert_eq!(buf[20], tokens::END_LIST);
        // Byte 21: EndOfData.
        assert_eq!(buf[21], tokens::END_OF_DATA);
    }

    #[test]
    fn revert_sp_call_with_keep_global_range_key_emits_named_arg() {
        let buf = encode_revert_sp_call(true);
        // After the two UIDs (offset 0..19) we expect:
        //   0xF0 StartList
        //   0xF2 StartName
        //   0x06 (named-arg index)
        //   0x01 (value = 1)
        //   0xF3 EndName
        //   0xF1 EndList
        //   0xF9 EndOfData
        assert_eq!(buf[19], tokens::START_LIST);
        assert_eq!(buf[20], tokens::START_NAME);
        assert_eq!(buf[21], 0x06);
        assert_eq!(buf[22], 0x01);
        assert_eq!(buf[23], tokens::END_NAME);
        assert_eq!(buf[24], tokens::END_LIST);
        assert_eq!(buf[25], tokens::END_OF_DATA);
    }
}
