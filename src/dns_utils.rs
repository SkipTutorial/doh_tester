pub fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::new();

    let id = 0x1234u16.to_be_bytes();
    packet.extend_from_slice(&id);

    let flags = 0x0100u16.to_be_bytes();
    packet.extend_from_slice(&flags);

    let qdcount = 1u16.to_be_bytes();
    packet.extend_from_slice(&qdcount);

    let ancount = 0u16.to_be_bytes();
    packet.extend_from_slice(&ancount);

    let nscount = 0u16.to_be_bytes();
    packet.extend_from_slice(&nscount);

    let arcount = 0u16.to_be_bytes();
    packet.extend_from_slice(&arcount);

    for label in domain.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);

    let qtype = 1u16.to_be_bytes();
    packet.extend_from_slice(&qtype);

    let qclass = 1u16.to_be_bytes();
    packet.extend_from_slice(&qclass);

    packet
}

pub fn parse_dns_response(data: &[u8]) -> Option<Vec<String>> {
    if data.len() < 12 {
        return None;
    }

    let qdcount = u16::from_be_bytes([data[4], data[5]]);
    let ancount = u16::from_be_bytes([data[6], data[7]]);
    if ancount == 0 {
        return None;
    }

    let mut ips = Vec::new();
    let mut offset = 12;

    // Skip the question section (QNAME + QTYPE + QCLASS for each question)
    for _ in 0..qdcount {
        if offset >= data.len() {
            return None;
        }
        let name_end = parse_name(data, offset)?;
        offset = name_end;
        // Skip QTYPE (2 bytes) + QCLASS (2 bytes)
        if offset + 4 > data.len() {
            return None;
        }
        offset += 4;
    }

    // Now parse the answer section
    for _ in 0..ancount {
        if offset >= data.len() {
            break;
        }

        let name_end = parse_name(data, offset)?;
        offset = name_end;

        if offset + 10 > data.len() {
            break;
        }

        let atype = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let _aclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        let _ttl = u32::from_be_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        let rdlength = u16::from_be_bytes([data[offset + 8], data[offset + 9]]);
        offset += 10;

        if atype == 1 && rdlength == 4 && offset + 4 <= data.len() {
            let octets = &data[offset..offset + 4];
            let ip = format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]);
            ips.push(ip);
        }

        offset += rdlength as usize;
    }

    Some(ips)
}

fn parse_name(data: &[u8], mut offset: usize) -> Option<usize> {
    loop {
        if offset >= data.len() {
            return None;
        }

        let len = data[offset];
        offset += 1;

        if len == 0 {
            break;
        }

        if (len & 0xC0) == 0xC0 {
            // Compressed name pointer — consume the second byte and stop
            if offset >= data.len() {
                return None;
            }
            offset += 1;
            break;
        }

        let label_len = len as usize;
        if offset + label_len > data.len() {
            return None;
        }
        offset += label_len;
    }

    Some(offset)
}

pub fn base64url_encode(data: &[u8]) -> String {
    const BASE64_ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut result = String::new();
    let mut buffer = 0u32;
    let mut bits = 0u8;

    for byte in data {
        buffer = (buffer << 8) | (*byte as u32);
        bits += 8;

        while bits >= 6 {
            bits -= 6;
            let index = ((buffer >> bits) & 0x3F) as usize;
            result.push(BASE64_ALPHABET[index] as char);
        }
    }

    if bits > 0 {
        let index = ((buffer << (6 - bits)) & 0x3F) as usize;
        result.push(BASE64_ALPHABET[index] as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64url_encode() {
        let input = b"hello";
        let encoded = base64url_encode(input);
        assert_eq!(encoded, "aGVsbG8");
    }

    #[test]
    fn test_build_dns_query() {
        let query = build_dns_query("example.com");
        assert!(!query.is_empty());
        assert_eq!(query[0], 0x12);
        assert_eq!(query[1], 0x34);
    }

    #[test]
    fn test_parse_dns_response() {
        // A standard DNS response for example.com with one A record answer
        // Header: ID=0x1234, Flags=0x8180, QDCOUNT=1, ANCOUNT=1, NSCOUNT=0, ARCOUNT=0
        // Question: example.com IN A
        // Answer: example.com IN A 60 147.24.34.52
        let response = [
            0x12, 0x34, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x07, 0x65,
            0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
            0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x04, 0x93,
            0x18, 0x22, 0x34,
        ];

        let ips = parse_dns_response(&response);
        assert!(ips.is_some());
        let ips = ips.unwrap();
        assert_eq!(ips.len(), 1);
        assert_eq!(ips[0], "147.24.34.52");
    }

    #[test]
    fn test_parse_dns_response_no_answers() {
        // Response with ANCOUNT=0
        let response = [
            0x12, 0x34, 0x81, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x65,
            0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00,
            0x01,
        ];

        let ips = parse_dns_response(&response);
        assert!(ips.is_none());
    }

    #[test]
    fn test_parse_dns_response_too_short() {
        let response = [0x12, 0x34, 0x81];
        let ips = parse_dns_response(&response);
        assert!(ips.is_none());
    }

    #[test]
    fn test_parse_name_simple() {
        // "example.com" encoded as DNS name: 07 example 03 com 00
        let data = [
            0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
        ];
        let end = parse_name(&data, 0);
        assert_eq!(end, Some(13));
    }

    #[test]
    fn test_parse_name_compressed() {
        // A pointer to offset 0x000c
        let data = [0xc0, 0x0c];
        let end = parse_name(&data, 0);
        assert_eq!(end, Some(2));
    }
}
