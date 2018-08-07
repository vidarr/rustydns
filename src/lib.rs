mod dns {

    use std::str;

#[test]

    fn test_calc_dns_string_len() {

        let v = vec![];

        assert_eq!(calc_dns_string_len(&v).unwrap(), 0);

        let v = vec![ [0u8; 64] ];
        assert_eq!(calc_dns_string_len(&v).unwrap(), 1);

        let v = vec![ [0u8; 64], [0u8; 64] ];
        assert_eq!(calc_dns_string_len(&v).unwrap(), 2);

        let v = vec![ [0u8; 64], [0u8; 64], [0u8; 64] ];
        assert_eq!(calc_dns_string_len(&v).unwrap(), 3);


        let v = dns_string_from_str("abcde").unwrap();
        assert_eq!(calc_dns_string_len(&vec![v]).unwrap(), 6);

        let w = dns_string_from_str("fgh").unwrap();
        assert_eq!(calc_dns_string_len(&vec![w, v]).unwrap(), 10);



    }

    fn dns_string_from_str(name : &str) -> Result<[u8; 64], ()> {

        let len = name.len();

        if 63 < len {
            return Err(())
        }

        let mut n :[u8; 64] = [0; 64];
        n[0] = len as u8;
        n[1 .. len + 1].clone_from_slice(name.as_bytes());
        Ok(n)
    }

    trait SerializableToBytes {

        fn to_bytes(&self, target_bytes : &mut [u8]) -> Result<Vec<u8>, ()>;

    }

    enum QuestionType {

        A = 1,
        NS = 2,
        CNAME = 5,
        PTR = 12,
        HINFO = 13,
        MX = 15,
        AXFR = 252,
        ANY = 255

    }

    /// Representation of a DNS question
    pub struct Question {

        name : Vec<[u8;64]>,
        qtype : QuestionType,
        qclass : u16

    }

    pub struct ResourceRecord {

        name : Vec<[u8;64]>

    }


    /// In-memory representation of a DNS message
    pub struct DnsMessage {

        id : u16,
        flags: u16,
        questions : Vec<Question>,
        answers : Vec<ResourceRecord>,
        authorities : Vec<ResourceRecord>,
        additionals : Vec<ResourceRecord>

    }

    fn calc_dns_string_len(dns_string : &Vec<[u8; 64]>) -> Result<usize, ()> {

        let mut len = dns_string.len();

        for label in dns_string {

            if label[0] > 63 {
                return Err(());
            }

            len += label[0] as usize;

        }

        Ok(len)

    }

    impl SerializableToBytes for Vec<[u8;64]> {

        fn to_bytes(&self, target_bytes : &mut [u8]) -> Result<Vec<u8>, ()> {

            let mut v : Vec<u8> = Vec::new();

            for dns_string in self {

                let len = dns_string[0] as usize;

                if 63 < len {
                    return Err(());
                }

                v.push(len as u8);
                v.extend_from_slice(&dns_string[1 .. len]);

            }

            Ok(v)

        }

    }


    // fn calc_len(message : &DnsMessage) -> Result<usize,()> {

    //     let mut len = 12;

    //     Ok(1)

    // }

}
