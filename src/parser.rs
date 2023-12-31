use crate::{batch::Batch, entry::Entry, line::Line};

const NEW_LINE: u8 = b'\n';
const EQUALS: u8 = b'=';

#[derive(PartialEq)]
enum Mode {
    Unknown,
    SingleLine,
    Multiline,
    LineEnd,
}

pub fn parse(bytes: Vec<u8>) -> Batch {
    let mut batch = Batch::new();
    let mut curr_line_bytes = Vec::new();
    let mut entry = Entry::new();
    let mut mode = Mode::Unknown;
    let mut last_byte_in_multiline: u8 = 0;

    for byte in bytes {
        match byte {
            EQUALS => {
                if mode == Mode::Unknown {
                    mode = Mode::SingleLine;
                }
                curr_line_bytes.push(byte);
            }
            NEW_LINE => {
                if mode == Mode::SingleLine {
                    mode = Mode::LineEnd;
                    entry.add_line(String::from_utf8_lossy(&curr_line_bytes).to_string().into());
                    curr_line_bytes = Vec::new();
                } else if mode == Mode::Unknown {
                    mode = Mode::Multiline;
                    curr_line_bytes.push(byte);
                } else if mode == Mode::Multiline {
                    if last_byte_in_multiline == NEW_LINE && byte == NEW_LINE {
                        mode = Mode::LineEnd;

                        entry.add_line(parse_multiline_line(curr_line_bytes));
                        curr_line_bytes = Vec::new();
                    } else {
                        curr_line_bytes.push(byte);
                    }
                } else if mode == Mode::LineEnd && byte == NEW_LINE {
                    batch.add_entry(entry);
                    entry = Entry::new();
                }
            }
            _ => {
                if mode == Mode::LineEnd {
                    mode = Mode::Unknown;
                }
                curr_line_bytes.push(byte)
            }
        }
        last_byte_in_multiline = byte;
    }

    if !curr_line_bytes.is_empty() {
        entry.add_line(String::from_utf8_lossy(&curr_line_bytes).to_string().into());
        batch.add_entry(entry);
    }

    batch
}

#[derive(PartialEq)]
enum MultiProcess {
    Start,
    KeyFound,
    LittleEndianFound,
    ValueFound,
}

fn parse_multiline_line(bytes: Vec<u8>) -> Line {
    let mut key = Vec::new();
    let mut value = Vec::new();
    let mut little_endian = Vec::new();
    let mut state = MultiProcess::Start;
    let mut counter = 0;

    for byte in &bytes[0..bytes.len() - 1] {
        if state == MultiProcess::Start {
            if *byte != NEW_LINE {
                key.push(*byte);
            } else {
                state = MultiProcess::KeyFound;
            }
        } else if state == MultiProcess::KeyFound {
            if counter != 7 {
                little_endian.push(*byte);
                counter += 1;
            } else {
                state = MultiProcess::LittleEndianFound;
            }
        } else if state == MultiProcess::LittleEndianFound {
            value.push(*byte);
        }
    }

    let key = String::from_utf8_lossy(&key).to_string();
    let value = String::from_utf8_lossy(&value).to_string();

    Line::new(key, value)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, ops::Deref};

    use super::*;

    fn get_simple_entry() -> Entry {
        let mut entry = Entry::new();
        entry.add_line(Line {
            key: "key".to_string(),
            value: "value".to_string(),
        });
        entry.add_line(Line {
            key: "key2".to_string(),
            value: "value2".to_string(),
        });
        entry.add_line(Line {
            key: "key3".to_string(),
            value: "value3".to_string(),
        });
        entry
    }

    #[test]
    fn simple() {
        let input = r#"key=value
key2=value2
key3=value3"#;
        let mut expected = Batch::new();
        expected.add_entry(get_simple_entry());

        let result = parse(input.as_bytes().to_vec());

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multiple_entries() {
        let input = r#"key=value
key2=value2
key3=value3

key=value
key2=value2
key3=value3"#;
        let mut expected = Batch::new();
        expected.add_entry(get_simple_entry());
        expected.add_entry(get_simple_entry());

        let result = parse(input.as_bytes().to_vec());

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_binary_serialization() {
        let input = r#"key=value
key2
       value2

key3=value3"#;
        let mut expected = Batch::new();
        expected.add_entry(get_simple_entry());

        let result = parse(input.as_bytes().to_vec());

        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_binary_serialization() {
        let input = r#"key=value
key2
       value2

key3=value3

key=value
key2
       value2

key3=value3"#;
        let mut expected = Batch::new();
        expected.add_entry(get_simple_entry());
        expected.add_entry(get_simple_entry());

        let result = parse(input.as_bytes().to_vec());

        assert_eq!(result, expected);
    }

    #[test]
    fn realworld_scenario() {
        let bytes: &mut Vec<u8> = &mut Vec::new();
        File::open("./tests/realworld_scenario")
            .unwrap()
            .read_to_end(bytes)
            .unwrap();

        let mut batch = Batch::new();
        let mut entry = Entry::new();
        entry.add_line("__CURSOR=s=70f49bfa32794d46b91517c6fe33e672;i=8437cf7;b=a7c9bca59fde4c7986b50521a1c3c669;m=daba1ac400;t=6013a2af74505;x=c0d8f741c8ddae0a".into());
        entry.add_line("__REALTIME_TIMESTAMP=1690199200843013".into());
        entry.add_line("__MONOTONIC_TIMESTAMP=939425186816".into());
        entry.add_line("_BOOT_ID=a7c9bca59fde4c7986b50521a1c3c669".into());
        entry.add_line("_TRANSPORT=stdout".into());
        entry.add_line("PRIORITY=6".into());
        entry.add_line("SYSLOG_FACILITY=3".into());
        entry.add_line("_UID=0".into());
        entry.add_line("_GID=0".into());
        entry.add_line("_CAP_EFFECTIVE=1ffffffffff".into());
        entry.add_line("_SELINUX_CONTEXT=unconfined".into());
        entry.add_line("_SYSTEMD_SLICE=system.slice".into());
        entry.add_line("_MACHINE_ID=9f9b27b97483564c8dcc3cf00d4583ae".into());
        entry.add_line("_HOSTNAME=somehost".into());
        entry.add_line("_STREAM_ID=bfe017276d894852814ae450b944cc4b".into());
        entry.add_line("SYSLOG_IDENTIFIER=some-bin".into());
        entry.add_line("_PID=837".into());
        entry.add_line("_COMM=some-bin".into());
        entry.add_line("_EXE=some-bin".into());
        entry.add_line("_CMDLINE=<some command line>".into());
        entry.add_line("_SYSTEMD_CGROUP=/system.slice/control-plane.service".into());
        entry.add_line("_SYSTEMD_UNIT=control-plane.service".into());
        entry.add_line("_SYSTEMD_INVOCATION_ID=6949c63262384d27871ca47184883067".into());
        entry.add_line("MESSAGE={\"timestamp\":\"2023-07-24T11:46:40.842837Z\",\"level\":\"INFO\",\"action\":\"check\",\"status\":\"ok\",\"duration\":0.261779671,\"block_height\":54663094,\"error\":\"None\"}".into());

        batch.add_entry(entry);

        let mut entry = Entry::new();
        entry.add_line("__CURSOR=s=70f49bfa32794d46b91517c6fe33e672;i=8437cf8;b=a7c9bca59fde4c7986b50521a1c3c669;m=daba1ad1b7;t=6013a2af752bb;x=b92b52ed8227368d".into());
        entry.add_line("__REALTIME_TIMESTAMP=1690199200846523".into());
        entry.add_line("__MONOTONIC_TIMESTAMP=939425190327".into());
        entry.add_line("_BOOT_ID=a7c9bca59fde4c7986b50521a1c3c669".into());
        entry.add_line("_TRANSPORT=stdout".into());
        entry.add_line("PRIORITY=6".into());
        entry.add_line("SYSLOG_FACILITY=3".into());
        entry.add_line("_UID=0".into());
        entry.add_line("_GID=0".into());
        entry.add_line("_CAP_EFFECTIVE=1ffffffffff".into());
        entry.add_line("_SELINUX_CONTEXT=unconfined".into());
        entry.add_line("_SYSTEMD_SLICE=system.slice".into());
        entry.add_line("_MACHINE_ID=9f9b27b97483564c8dcc3cf00d4583ae".into());
        entry.add_line("_HOSTNAME=somehost".into());
        entry.add_line("_STREAM_ID=bfe017276d894852814ae450b944cc4b".into());
        entry.add_line("SYSLOG_IDENTIFIER=some-bin".into());
        entry.add_line("_PID=837".into());
        entry.add_line("_COMM=some-bin".into());
        entry.add_line("_EXE=some-bin".into());
        entry.add_line("_CMDLINE=<some command line>".into());
        entry.add_line("_SYSTEMD_CGROUP=/system.slice/control-plane.service".into());
        entry.add_line("_SYSTEMD_UNIT=control-plane.service".into());
        entry.add_line("_SYSTEMD_INVOCATION_ID=6949c63262384d27871ca47184883067".into());
        entry.add_line("MESSAGE={\"timestamp\":\"2023-07-24T11:46:40.846352Z\",\"level\":\"INFO\",\"action\":\"check\",\"status\":\"ok\",\"duration\":0.275869882,\"block_height\":46662909,\"error\":\"None\"}".into());

        batch.add_entry(entry);

        let result = parse(bytes.deref().to_vec());
        assert_eq!(result, batch);
    }
}
