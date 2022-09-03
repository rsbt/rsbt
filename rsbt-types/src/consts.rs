/// BitTorrent Handshake.
///
/// The handshake starts with character ninteen (decimal) followed by the string 'BitTorrent protocol'.
/// The leading character is a length prefix, put there in the hope that other new protocols may do the same and thus be trivially distinguishable from each other.
///
/// All later integers sent in the protocol are encoded as four bytes big-endian.
///
/// After the fixed headers come eight reserved bytes, which are all zero in all current implementations.
/// If you wish to extend the protocol using these bytes, please coordinate with Bram Cohen to make sure all extensions are done compatibly.
pub const BITTORRENT_HANDSHAKE: [u8; 28] =
    *b"\x13BitTorrent protocol\x00\x00\x00\x00\x00\x00\x00\x00";
