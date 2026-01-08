use std::vec;

use bytes::{Bytes, BytesMut}; // get_u8 

#[derive(Clone, Debug)]
pub enum RespType {
    // Number(usize),
    Command(Vec<RespType>),
    SimpleString(String),
    BulkString(String),
    SimpleError(String),
}

impl RespType {
    pub fn parse(buffer: &BytesMut) -> RespType {
        let c = buffer[0] as char;
        return match c {
            '*' => Self::parse_command(buffer),
            _ => Self::invalid_command().0,
        };
    }

    pub fn sub_parse(buffer: &BytesMut) -> (RespType, usize) {
        let c = buffer[0] as char;
        return match c {
            // '*' => Self::parse_command(buffer),
            '$' => Self::parse_bulk_string(buffer),
            '+' => Self::parse_simple_string(buffer),
            _ => Self::invalid_command(),
        };
    }

    pub fn invalid_command() -> (RespType, usize) {
        (RespType::SimpleError(String::from("Invalid Command")), 0)
    }

    pub fn parse_command(buf: &BytesMut) -> RespType {
        let mut buffer = buf.clone();
        let (_, bytes_consumed) =
            if let Some((buf_data, len)) = Self::read_till_crlf(&buffer[1..]) {
                let command_len = Self::parse_usize_from_buf(buf_data);
                (command_len, len + 1)
            } else {
                return Self::invalid_command().0;
            };

        let mut command = vec![];

        buffer = buffer.split_off(bytes_consumed);

        while buffer.len() > 0 {
            let (value, n) = Self::sub_parse(&buffer);
            command.push(value);
            buffer = buffer.split_off(n);
        }

        RespType::Command(command)
    }

    pub fn parse_bulk_string(buffer: &BytesMut) -> (RespType, usize) {
        let (bulk_str_len, bytes_consumed) =
            if let Some((buf_data, len)) = Self::read_till_crlf(&buffer[1..]) {
                let bulk_str_len = Self::parse_usize_from_buf(buf_data);
                (bulk_str_len, len + 1)
            } else {
                return Self::invalid_command();
            };

        let bulk_str_end_idx = bytes_consumed + bulk_str_len as usize;
        if bulk_str_end_idx >= buffer.len() {
            return Self::invalid_command();
        }

        let bulk_str = String::from_utf8(buffer[bytes_consumed..bulk_str_end_idx].to_vec());

        match bulk_str {
            Ok(bs) => (RespType::BulkString(bs), bulk_str_end_idx + 2),
            Err(_) => Self::invalid_command(),
        }
    }

    pub fn parse_simple_string(buffer: &BytesMut) -> (RespType, usize) {
        // read until CRLF and parse the bytes into an UTF-8 string.
        if let Some((buf_data, len)) = Self::read_till_crlf(&buffer[1..]) {
            let utf8_str = String::from_utf8(buf_data.to_vec());

            return match utf8_str {
                Ok(simple_str) => (RespType::SimpleString(simple_str), len + 1),
                Err(_) => {
                    return Self::invalid_command();
                }
            };
        }

        Self::invalid_command()
    }

    fn read_till_crlf(buf: &[u8]) -> Option<(&[u8], usize)> {
        for i in 1..buf.len() {
            if buf[i - 1] == b'\r' && buf[i] == b'\n' {
                return Some((&buf[0..(i - 1)], i + 1));
            }
        }

        None
    }

    pub fn to_bytes(&self) -> Bytes {
        return match self {
            RespType::SimpleString(ss) => Bytes::from_iter(format!("+{}\r\n", ss).into_bytes()),
            RespType::BulkString(bs) => {
                let bulk_str_bytes = format!("${}\r\n{}\r\n", bs.chars().count(), bs).into_bytes();
                Bytes::from_iter(bulk_str_bytes)
            }
            RespType::SimpleError(es) => Bytes::from_iter(format!("-{}\r\n", es).into_bytes()),
            // RespType::Number(u)=>Bytes::from_iter()
            RespType::Command(command) => {
                let mut command_str = format!("*{}\r\n", command.len());

                for i in 0..command.len() {
                    let command_byte = command[i].to_bytes();

                    command_str.push_str(str::from_utf8(&command_byte).unwrap());
                }
                Bytes::from_iter(command_str.into_bytes())
            }
        };
    }

    pub fn to_readable_string(&self) -> String {
        return match self {
            RespType::SimpleString(ss) => ss.to_string(),
            RespType::BulkString(bs) => bs.to_string(),
            RespType::SimpleError(es) => es.to_string(),
            RespType::Command(command) => {
                let mut command_str = String::new();

                for i in 0..command.len() {
                    command_str.push_str(&command[i].to_readable_string());
                    command_str.push_str(" ");
                }

                command_str
            }
        };
    }

    fn parse_usize_from_buf(buf: &[u8]) -> usize {
        let utf8_str = String::from_utf8(buf.to_vec());
        let parsed_int = match utf8_str {
            Ok(s) => {
                let int = s.parse::<usize>();
                match int {
                    Ok(n) => n,
                    Err(_) => Self::invalid_command().1,
                }
            }
            Err(_) => Self::invalid_command().1,
        };

        parsed_int
    }
}

impl RespType {
    pub fn str_to_resp_byte(value: &str) -> Bytes {
        let resp = RespType::SimpleString(String::from(value));

        resp.to_bytes()
    }
}
