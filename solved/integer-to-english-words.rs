impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        static BASE: [&'static str; 20] = [
            "", "One", "Two", "Three", "Four", "Five",
            "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
            "Sixteen", "Seventeen", "Eighteen", "Nineteen"
        ];
        static TENS: [&'static str; 10] = [
            "", "Ten", "Twenty", "Thirty", "Forty",
            "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"
        ];
        static LEVEL: [&'static str; 4] = [
            "", " Thousand", " Million", " Billion"
        ];
        fn num_to_words(n: usize, level: usize) -> String {
            let mut s;
            if n >= 1000 {
                s = num_to_words(n / 1000, level + 1);
            } else {
                s = String::with_capacity(64);
            }
            let mut n = n % 1000;
            if n != 0 {
                if s.len() > 0 {
                    s.push(' ');
                }
                if n >= 100 {
                    s.push_str(BASE[n / 100]);
                    s.push_str(" Hundred");
                    n %= 100;
                    if n != 0 {
                        s.push(' ');
                    }
                }
                if n < BASE.len() {
                    s.push_str(BASE[n]);
                } else {
                    s.push_str(TENS[n / 10]);
                    n %= 10;
                    if n != 0 {
                        s.push(' ');
                        s.push_str(BASE[n % 10]);
                    }
                }
                s.push_str(LEVEL[level]);
            }
            s
        }
        num_to_words(num as usize, 0)
    }
}
