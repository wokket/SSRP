/// Helper methods to build/parse SSRP packets.  Actual networking stuff is elsewhere.
use std::str;

const NUL: u8 = 0x00;
const CLNT_UCAST_EX: u8 = 0x03;
const CLNT_UCAST_INST: u8 = 0x04;
const SVR_RESP: u8 = 0x05;

/// creates a [`CLNT_UCAST_INST` message](https://docs.microsoft.com/en-us/openspecs/windows_protocols/mc-sqlr/c97b04b5-d80f-4d3e-9195-83bbfe246639)
/// suitable for sending over the network.  
/// This packet requests resolution/connection info for a specific named instance.
pub fn get_instance_request(instance_name: &str) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.push(CLNT_UCAST_INST);

    instance_name
        .as_bytes()
        .iter()
        .for_each(|b| buffer.push(*b));

    buffer.push(NUL); //NUL terminate the instance name
    buffer
}

/// This request can be sent to a server to get a set of all the instance
/// details available on that server ([`CLNT_UCAST_EX` message](https://docs.microsoft.com/en-us/openspecs/windows_protocols/mc-sqlr/ee0e41b0-204f-4a95-b8bd-5783a7c72cb2)).
pub const fn get_unicast_browse_request() -> [u8; 1] {
    [CLNT_UCAST_EX]
}

pub fn parse_server_response<'a>(data: &'a [u8]) -> ServerResponse<'a> {
    assert_eq!(data[0], SVR_RESP);

    let mut arr: [u8; 2] = [0, 0];
    arr.copy_from_slice(&data[1..=2]);
    let _length = u16::from_le_bytes(arr);

    let data = str::from_utf8(&data[3..]).unwrap();

    ServerResponse { data }
}

/// This struct represents the results of a call to the SQL Browser
/// Service (ie a [`SVR_RESP` SSRP](https://docs.microsoft.com/en-us/openspecs/windows_protocols/mc-sqlr/2e1560c9-5097-4023-9f5e-72b9ff1ec3b1) message).
pub struct ServerResponse<'a> {
    pub data: &'a str,
}

mod tests {
    #[test]
    fn ensure_instance_request_builds_correctly() {
        let expected = vec![0x04, 0x41, 0];
        let actual = super::get_instance_request("A");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_response_parser() {
        let input = [
            5, 87, 0, 83, 101, 114, 118, 101, 114, 78, 97, 109, 101, 59, 84, 65, 78, 71, 79, 51,
            59, 73, 110, 115, 116, 97, 110, 99, 101, 78, 97, 109, 101, 59, 83, 81, 76, 69, 88, 80,
            82, 69, 83, 83, 59, 73, 115, 67, 108, 117, 115, 116, 101, 114, 101, 100, 59, 78, 111,
            59, 86, 101, 114, 115, 105, 111, 110, 59, 49, 53, 46, 48, 46, 50, 48, 48, 48, 46, 53,
            59, 116, 99, 112, 59, 49, 50, 56, 53, 59, 59,
        ];
        let actual = super::parse_server_response(&input);

        println!("Data: {:?}", actual.data);
        assert_eq!(actual.data.len(), 87); //the length is stored in bytes 1,2 above.

        // The data should terminate with two semicolons
        assert_eq!(actual.data[actual.data.len() - 2..], *";;");
    }
}
