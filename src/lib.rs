
mod tests {
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub mod inpx{
    pub mod int {
        use std::io::stdin;

        static WRN: &str = "Could not read";

        fn test() {
            println!("Test");
        }

        pub fn parse_i8() -> i8 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<i8>().unwrap()
        }

        pub fn parse_u8() -> u8 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<u8>().unwrap()
        }

        pub fn parse_i16() -> i16 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<i16>().unwrap()
        }

        pub fn parse_u16() -> u16 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<u16>().unwrap()
        }
    
        pub fn parse_i32() -> i32 {
            let mut s: String = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<i32>().unwrap()
        }

        pub fn parse_u32() -> u32 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<u32>().unwrap()
        }

        pub fn parse_i64() -> i64 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<i64>().unwrap()
        }

        pub fn parse_u64() -> u64 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<u64>().unwrap()
        }

        pub fn parse_i128() -> i128 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<i128>().unwrap()
        }

        pub fn parse_u128() -> u128 {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<u128>().unwrap()
        }

        pub fn parse_isize() -> isize {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<isize>().unwrap()
        }

        pub fn parse_usize() -> usize {
            let mut s = String::new();
            stdin().read_line(&mut s).expect(WRN);
            s.trim_end().parse::<usize>().unwrap()
        }
    }
}
