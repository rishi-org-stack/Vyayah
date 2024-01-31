// +acc/r/n<size_of_buffer>/r/n<endcoded_buff>
// +txn/r/n<size_of_buffer>/r/n<endcoded_buff>
// -acc/r/n<size_of_buffer>/r/n<endcoded_buff>
// $acc/r/n<size_of_buffer>/r/n<endcoded_buff(name_of_ccount)>
// $txn/r/n<size_of_buffer>/r/n<endcoded_buff(id_txn)>
pub mod request {
    fn separate(raw: &Vec<u8>, size: usize) -> Vec<Vec<u8>> {
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

    impl Function {
        pub fn parse(raw: &Vec<u8>) -> Result<Function, Error> {
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

    #[test]
    fn check_separate() {
        let raw = "ok\r\njava\r\noko\r\n".as_bytes();
        let separate = separate(&raw.to_vec(), raw.len());
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
        let function = Function::parse(&raw.as_bytes().to_vec()).unwrap();
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
        let function = Function::parse(&raw.as_bytes().to_vec()).unwrap();
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
        let function = Function::parse(&raw.as_bytes().to_vec()).unwrap();
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
        let function = Function::parse(&raw.as_bytes().to_vec()).unwrap();
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
        let function = Function::parse(&raw.as_bytes().to_vec()).unwrap();
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

pub mod response {
    // ERR\r\n<buffer_size>\r\n<endcoded_buff_of_response>
    // MSG\r\n<buffer_size>\r\n<endcoded_buff_of_response>
    fn usize_to_vec_u8(value: u16) -> Vec<u8> {
        let mut vec_u8: Vec<u8> = Vec::new();
        let mut n = value.clone();

        if n < 10 {
            vec_u8.push((n + 48) as u8);
        }

        while n > 0 {
            let x = n % 10;
            vec_u8.push((x + 48) as u8);
            n -= x;
            n = n / 10;
        }
        vec_u8.reverse();
        vec_u8
    }
    pub enum ResponseType {
        ERR,
        MSG,
    }

    pub struct Response {
        kind: ResponseType,
        size: usize,
        buff: Vec<u8>,
    }

    impl Response {
        pub fn new(kind: ResponseType, size: usize, buff: Vec<u8>) -> Response {
            Response { kind, size, buff }
        }

        pub fn to_vec(&self) -> Vec<u8> {
            let mut encoded_response: Vec<u8> = Vec::new();
            let response_type = match self.kind {
                ResponseType::ERR => "ERR",
                ResponseType::MSG => "MSG",
            };
            let mut response_vec_u8 = response_type.as_bytes().to_vec();
            let size_vec_u8 = usize_to_vec_u8(self.size as u16);
            let mut buff_copy = self.buff.clone();

            encoded_response.append(&mut response_vec_u8);
            encoded_response.push(13);
            encoded_response.push(10);

            encoded_response.append(&mut size_vec_u8.to_vec());
            encoded_response.push(13);
            encoded_response.push(10);

            encoded_response.append(&mut buff_copy);
            encoded_response.push(13);
            encoded_response.push(10);

            encoded_response
        }
    }

    #[test]
    fn usize_to_vec_u8_test() {
        //success
        let expected: Vec<u8> = vec![53, 49, 48];
        let result = usize_to_vec_u8(510);
        assert_eq!(expected, result);

        //success
        let expected: Vec<u8> = vec![48];
        let result = usize_to_vec_u8(0);
        assert_eq!(expected, result);

        //success
        let expected: Vec<u8> = vec![49, 48];
        let result = usize_to_vec_u8(10);
        assert_eq!(expected, result);
    }
    #[test]
    fn err_response() {
        let expected = "ERR\r\n10\r\nrishijhaoi\r\n".as_bytes().to_vec();

        let response = Response::new(ResponseType::ERR, 10, "rishijhaoi".as_bytes().to_vec());
        let result = response.to_vec();

        assert_eq!(result, expected);

        let expected = "MSG\r\n10\r\nrishijhaoi\r\n".as_bytes().to_vec();

        let response = Response::new(ResponseType::MSG, 10, "rishijhaoi".as_bytes().to_vec());
        let result = response.to_vec();

        assert_eq!(result, expected);
    }
}
