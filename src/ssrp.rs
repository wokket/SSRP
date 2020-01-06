/// Helper methods to build/parse SSRP packets.

/// creates a CLNT_UCAST_INST packet suitable for sending over the network.  
/// This packet requests resolution/connection info for a specific named instance.
pub fn get_instance_request(instance_name: &str) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.push(0x04); //CLNT_UCAST_INST

    instance_name
        .as_bytes()
        .iter()
        .for_each(|b| buffer.push(*b));

    buffer.push(0); //NUL terminate the instance name
    buffer
}

pub fn parse_server_response(data: &[u8]) {}

mod tests {
    #[test]
    fn ensure_instance_request_builds_correctly() {
        let expected = vec![0x04, 0x41, 0];
        let actual = super::get_instance_request("A");
        assert_eq!(expected, actual);
    }
}
