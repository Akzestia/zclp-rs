use std::vec::Vec;

#[allow(dead_code)]
struct VariableLengthInteger {
    len: u8,
    value: Vec<u8>, // ((2^Len)*8)-2 bits
}

impl VariableLengthInteger {
    pub fn new() {
        Self { len: 0, value: 0 }
    }

    pub fn set(&mut self, value: u64) {}
}

#[allow(dead_code)]
mod frames {
    use super::VariableLengthInteger;
    use std::option::Option;
    use std::vec::Vec;

    pub enum Frames {
        Padding(Padding),
        Ping(Ping),
        Ack(Ack),
        ResetStream(ResetStream),
        StopSending(StopSending),
        Crypto(Crypto),
        NewToken(NewToken),
        Stream(Stream),
        MaxData(MaxData),
        MaxStreamData(MaxStreamData),
        MaxStreams(MaxStreams),
        DataBlocked(DataBlocked),
        StreamDataBlocked(StreamDataBlocked),
        StreamsBlocked(StreamsBlocked),
        ConnectionClose(ConnectionClose),
        HandShakeDone(HandShakeDone),
    }

    pub struct Padding {
        type_: VariableLengthInteger, // 0
    }

    pub struct Ping {
        type_: VariableLengthInteger, // 1
    }

    pub struct AckRange {
        gap: VariableLengthInteger,   // 1
        range: VariableLengthInteger, // 1
    }

    pub struct EcnCount {
        ect0: VariableLengthInteger,
        ect1: VariableLengthInteger,
        ecnce: VariableLengthInteger,
    }

    pub struct Ack {
        type_: VariableLengthInteger, // 3 | 4
        largest_ack_num: VariableLengthInteger,
        delay: VariableLengthInteger,
        range_count: VariableLengthInteger,
        ranges: Vec<AckRange>,
        ecn_count: Option<EcnCount>, // FT.Value == 3
    }

    pub struct ResetStream {
        type_: VariableLengthInteger, // 4
        stream_id: VariableLengthInteger,
        error_code: VariableLengthInteger,
        final_size: VariableLengthInteger,
    }

    pub struct StopSending {
        type_: VariableLengthInteger, // 5;
        stream_id: VariableLengthInteger,
        error_code: VariableLengthInteger,
    }

    pub struct Crypto {
        type_: VariableLengthInteger, // 6;
        offset: VariableLengthInteger,
        length: VariableLengthInteger,
    }

    pub struct NewToken {
        type_: VariableLengthInteger, // 7;
        length: VariableLengthInteger,
        token: Vec<u8>,
    }

    pub struct Stream {
        unused: u8, // 1
        stream_id: VariableLengthInteger,
        length: VariableLengthInteger,
        stream_data: Vec<u8>,
    }

    pub struct MaxData {
        type_: VariableLengthInteger, // 16
        max_data: VariableLengthInteger,
    }

    pub struct MaxStreamData {
        type_: VariableLengthInteger, // 17
        stream_id: VariableLengthInteger,
        max_stream_data: VariableLengthInteger,
    }

    pub struct MaxStreams {
        type_: VariableLengthInteger, // 18 || 19
        max_streams: VariableLengthInteger,
    }

    pub struct DataBlocked {
        type_: VariableLengthInteger, // 20
        data_limit: VariableLengthInteger,
    }

    pub struct StreamDataBlocked {
        type_: VariableLengthInteger, // 21
        stream_id: VariableLengthInteger,
        stream_data_limit: VariableLengthInteger,
    }

    pub struct StreamsBlocked {
        type_: VariableLengthInteger, // 22 || 23
        stream_limit: VariableLengthInteger,
    }

    pub struct ConnectionClose {
        type_: VariableLengthInteger, // 28 || 29
        error: VariableLengthInteger,
        frame_type: VariableLengthInteger,
        phrase_len: VariableLengthInteger,
        phrase: u8,
    }

    pub struct HandShakeDone {
        type_: VariableLengthInteger, // 30
    }
}

#[allow(dead_code)]
mod packets {
    use super::VariableLengthInteger;
    use super::frames::Frames;

    #[repr(packed)]
    pub struct StatelessReset {
        header_form: u8,
        fixed_bit: u8,
        upredictable_bits: Vec<u64>,
        reset_token: [u8; 16],
    }

    #[repr(packed)]
    pub struct VersionNegotiation {
        header_form: u8,
        unused: u8,
        version_id: u32,
        destination_connection_id: u32,
        source_connection_id: u32,
        supported_versions: Vec<u32>,
    }

    #[repr(packed)]
    pub struct LongHeader {
        header_form: u8,
        fixed_bit: u8,
        packet_type: u8,
        reserved_bits: u8,
        packet_number_length: u8,
        version_id: u32,
        destination_connection_id: u32,
        source_connection_id: u32,
    }

    #[repr(packed)]
    pub struct Initial {
        header: LongHeader,
        token_length: VariableLengthInteger,
        token: Vec<u8>,
        length: VariableLengthInteger,
        packet_number: u8,
        payload: Frames, // Frames
    }

    #[repr(packed)]
    pub struct ShortHeader {
        header_form: u8,
        fixed_bit: u8,
        spin_bit: u8,
        reserved_bits: u8,
        key_phase: u8,
        packet_number_length: u8,
        destination_connection: u32,
        packet_number: u8,
        payload: Frames, // Frames
    }
}
