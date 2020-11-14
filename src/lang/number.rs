use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    INT(i32),
    FLOAT(f32),
}

impl Number {
    fn as_float(self) -> f32 {
        match self {
            Number::INT(integer) => integer as f32,
            Number::FLOAT(float) => float,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Number::INT(integer) => format!("{:#}", integer),
            Number::FLOAT(float) => format!("{:#}", float),
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        Number::FLOAT(self.as_float().powf(rhs.as_float()))
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::INT(int_a), Number::INT(int_b)) => Number::INT(int_a + int_b),
            (Number::FLOAT(float_a), Number::INT(int_b)) => Number::FLOAT(float_a + int_b as f32),
            (Number::INT(int_a), Number::FLOAT(float_b)) => Number::FLOAT(int_a as f32 + float_b),
            (Number::FLOAT(float_a), Number::FLOAT(float_b)) => Number::FLOAT(float_a + float_b),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::INT(int_a), Number::INT(int_b)) => Number::INT(int_a - int_b),
            (Number::FLOAT(float_a), Number::INT(int_b)) => Number::FLOAT(float_a - int_b as f32),
            (Number::INT(int_a), Number::FLOAT(float_b)) => Number::FLOAT(int_a as f32 - float_b),
            (Number::FLOAT(float_a), Number::FLOAT(float_b)) => Number::FLOAT(float_a - float_b),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::INT(int_a), Number::INT(int_b)) => Number::INT(int_a * int_b),
            (Number::FLOAT(float_a), Number::INT(int_b)) => Number::FLOAT(float_a * int_b as f32),
            (Number::INT(int_a), Number::FLOAT(float_b)) => Number::FLOAT(int_a as f32 * float_b),
            (Number::FLOAT(float_a), Number::FLOAT(float_b)) => Number::FLOAT(float_a * float_b),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Number::FLOAT(self.as_float() / rhs.as_float())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n_int(int: i32) -> Number {
        Number::INT(int)
    }

    fn n_float(float: f32) -> Number {
        Number::FLOAT(float)
    }

    #[test]
    fn calculate_add_correctly() {
        assert_eq!(n_int(1) + n_int(2), n_int(3));
        assert_eq!(n_int(1) + n_float(4.5), n_float(5.5));
        assert_eq!(n_float(8.0) + n_int(7), n_float(15.0));
        assert_eq!(n_float(8.0) + n_float(9.0), n_float(17.0));
    }

    #[test]
    fn calculate_sub_correctly() {
        assert_eq!(n_int(3) - n_int(2), n_int(1));
        assert_eq!(n_int(8) - n_float(4.5), n_float(3.5));
        assert_eq!(n_float(5.0) - n_int(7), n_float(-2.0));
        assert_eq!(n_float(8.0) - n_float(9.0), n_float(-1.0));
    }

    #[test]
    fn calculate_mul_correctly() {
        assert_eq!(n_int(3) * n_int(2), n_int(6));
        assert_eq!(n_int(8) * n_float(4.5), n_float(36.0));
        assert_eq!(n_float(5.2) * n_int(7), n_float(36.399998));
        assert_eq!(n_float(8.0) * n_float(9.0), n_float(72.0));
    }

    #[test]
    fn calculate_div_correctly() {
        assert_eq!(n_int(6) / n_int(3), n_float(2.0));
        assert_eq!(n_float(100.0) / n_float(3.0), n_float(33.333333333));
    }

    #[test]
    fn calculate_pow_correctly() {
        assert_eq!(n_int(10).pow(n_int(3)), n_float(1000.0));
        assert_eq!(n_int(3).pow(n_float(2.0)), n_float(9.0));
        assert_eq!(n_float(2.5).pow(n_float(7.45)), n_float(921.8346));
    }
}
