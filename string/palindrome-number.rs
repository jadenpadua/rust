impl Solution1 {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let (mut y, mut z): (i32, i32) = (x, 0);
        while y > 0 {
            z = z * 10 + y % 10;
            y /= 10;
        }
        return x == z;
    }
}

impl Solution2 {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string().chars().rev().eq(x.to_string().chars());
    }
}
