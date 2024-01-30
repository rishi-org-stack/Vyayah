// +acc/r/n<size_of_buffer>/r/n<endcoded_buff>
// +txn/r/n<size_of_buffer>/r/n<endcoded_buff>
// -acc/r/n<size_of_buffer>/r/n<endcoded_buff>
// $acc/r/n<size_of_buffer>/r/n<endcoded_buff(name_of_ccount)>
// $txn/r/n<size_of_buffer>/r/n<endcoded_buff(id_txn)>

pub enum FunctionTypes {
    Add,
    Delete,
    Describe,
}

pub enum EntityTypes {
    TXN,
    ACC,
}

#[derive(Debug)]
pub enum Error {
    InvalidEncodedString(EncodeStringError),
}

#[derive(Debug)]
pub enum EncodeStringError {
    Invalid,
    InvalidFunction,
    InvalidSize,
    InvalidEntityEncoding,
}

pub struct Function {
    pub function: FunctionTypes,
    pub entity: EntityTypes,
    pub size: usize,
    pub buffer: Vec<u8>,
}

fn separate(raw: Vec<u8>, size: usize) -> Vec<Vec<u8>> {
    let mut separate: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    let mut current: Vec<u8> = Vec::new();
    while i < size {
        if raw[i] == 13 && raw[i + 1] == 10 {
            i += 2;
            separate.push(current);
            current = Vec::new();
        } else {
            current.push(raw[i]);
            i += 1
        }
    }
    separate
}

fn vec_u8_to_usize(num_byte: &Vec<u8>, size: usize) -> Option<usize> {
    let mut num: usize = 0;
    let mut i = 0;
    while i < size {
        if num_byte[i] > 47 && num_byte[i] < 58 {
            num += (num_byte[i] - 48) as usize;
            num *= 10;
            i += 1
        } else {
            return None;
        }
    }

    // let num = num_byte.iter().fold(0 as u32, |mut acc, c| {
    //     acc += ((c - 47) * 10) as u32;
    //     acc
    // });
    Some(num / 10)
}

impl Function {
    pub fn parse(raw: Vec<u8>) -> Result<Function, Error> {
        let raw_size = raw.len();
        if raw_size == 0 {
            return Err(Error::InvalidEncodedString(EncodeStringError::Invalid));
        }

        let params: Vec<Vec<u8>> = separate(raw, raw_size);

        if params.len() != 3 {
            return Err(Error::InvalidEncodedString(EncodeStringError::Invalid));
        }

        let mut params_iter = params.iter();
        let function_entity_str = params_iter.nth(0).unwrap();
        let function_byte = match function_entity_str.iter().nth(0) {
            Some(b) => b,
            None => {
                return Err(Error::InvalidEncodedString(
                    EncodeStringError::InvalidFunction,
                ))
            }
        };

        let entity = &function_entity_str[1..];

        let msg_size_byte = params_iter.nth(0).unwrap();

        let msg_size = match vec_u8_to_usize(msg_size_byte, msg_size_byte.len()) {
            Some(v) => v,
            None => return Err(Error::InvalidEncodedString(EncodeStringError::InvalidSize)),
        };

        let mut msg_buff: Vec<u8> = vec![0; msg_size];
        msg_buff.clone_from_slice(params_iter.nth(0).unwrap());

        const ACC: &[u8] = "acc".as_bytes();
        const TXN: &[u8] = "txn".as_bytes();
        let function: Function = match function_byte {
            b'+' => match entity {
                ACC => Function {
                    function: FunctionTypes::Add,
                    entity: EntityTypes::ACC,
                    size: msg_size,
                    buffer: msg_buff,
                },

                TXN => Function {
                    function: FunctionTypes::Add,
                    entity: EntityTypes::TXN,
                    size: msg_size,
                    buffer: msg_buff,
                },

                _ => {
                    return Err(Error::InvalidEncodedString(
                        EncodeStringError::InvalidEntityEncoding,
                    ))
                }
            },
            b'-' => match entity {
                ACC => Function {
                    function: FunctionTypes::Delete,
                    entity: EntityTypes::ACC,
                    size: msg_size,
                    buffer: msg_buff,
                },
                _ => {
                    return Err(Error::InvalidEncodedString(
                        EncodeStringError::InvalidEntityEncoding,
                    ))
                }
            },

            b'$' => match entity {
                ACC => Function {
                    function: FunctionTypes::Describe,
                    entity: EntityTypes::ACC,
                    size: msg_size,
                    buffer: msg_buff,
                },

                TXN => Function {
                    function: FunctionTypes::Describe,
                    entity: EntityTypes::TXN,
                    size: msg_size,
                    buffer: msg_buff,
                },

                _ => {
                    return Err(Error::InvalidEncodedString(
                        EncodeStringError::InvalidEntityEncoding,
                    ))
                }
            },

            _ => todo!(),
        };

        Ok(function)
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{EntityTypes, FunctionTypes};

    use super::{separate, Function};

    #[test]
    fn check_separate() {
        let raw = "ok\r\njava\r\noko\r\n".as_bytes();
        let separate = separate(raw.to_vec(), raw.len());
        let expected = vec!["ok".as_bytes(), "java".as_bytes(), "oko".as_bytes()];
        let separate_size = separate.len();
        let expected_size = expected.len();

        assert_eq!(separate_size, expected_size);

        for i in 0..expected_size {
            assert_eq!(separate[i], expected[i])
        }
    }

    #[test]
    fn check_function_parse_acc() {
        let raw = "+acc\r\n13\r\nsomencodedstr\r\n";
        let function = Function::parse(raw.as_bytes().to_vec()).unwrap();
        let same_fn_type = match function.function {
            FunctionTypes::Add => true,
            _ => false,
        };
        assert_eq!(same_fn_type, true);

        let same_entity_type = match function.entity {
            EntityTypes::ACC => true,
            _ => false,
        };

        assert_eq!(same_entity_type, true);
        assert_eq!(function.size, 13);
        assert_eq!(function.buffer, "somencodedstr".as_bytes().to_vec());

        // Delete account
        let raw = "-acc\r\n13\r\nsomencodedstr\r\n";
        let function = Function::parse(raw.as_bytes().to_vec()).unwrap();
        let same_fn_type = match function.function {
            FunctionTypes::Delete => true,
            _ => false,
        };
        assert_eq!(same_fn_type, true);

        let same_entity_type = match function.entity {
            EntityTypes::ACC => true,
            _ => false,
        };

        assert_eq!(same_entity_type, true);

        // Describe account
        let raw = "$acc\r\n13\r\nsomencodedstr\r\n";
        let function = Function::parse(raw.as_bytes().to_vec()).unwrap();
        let same_fn_type = match function.function {
            FunctionTypes::Describe => true,
            _ => false,
        };
        assert_eq!(same_fn_type, true);

        let same_entity_type = match function.entity {
            EntityTypes::ACC => true,
            _ => false,
        };

        assert_eq!(same_entity_type, true);
    }
    #[test]
    fn check_function_parse_txn() {
        let raw = "+txn\r\n13\r\nsomencodedstr\r\n";
        let function = Function::parse(raw.as_bytes().to_vec()).unwrap();
        let same_fn_type = match function.function {
            FunctionTypes::Add => true,
            _ => false,
        };
        assert_eq!(same_fn_type, true);

        let same_entity_type = match function.entity {
            EntityTypes::TXN => true,
            _ => false,
        };

        assert_eq!(same_entity_type, true);
        assert_eq!(function.size, 13);
        assert_eq!(function.buffer, "somencodedstr".as_bytes().to_vec());

        // Describe transaction
        let raw = "$txn\r\n13\r\nsomencodedstr\r\n";
        let function = Function::parse(raw.as_bytes().to_vec()).unwrap();
        let same_fn_type = match function.function {
            FunctionTypes::Describe => true,
            _ => false,
        };
        assert_eq!(same_fn_type, true);

        let same_entity_type = match function.entity {
            EntityTypes::TXN => true,
            _ => false,
        };

        assert_eq!(same_entity_type, true);
    }
}
