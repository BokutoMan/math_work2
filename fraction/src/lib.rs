use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};
use std::cmp::{Ordering, PartialOrd};

#[derive(Copy, Clone)]
pub struct Fraction{
    pub numerator:u64,
    pub denominator:u64,
    pub positive:bool
}

impl Fraction {
    fn gcd(&mut self){
        let mut a = self.numerator;
        let mut b = self.denominator;
        if a*b == 0 {
            self.numerator = 0;
            self.denominator = 1;
            self.positive = true;
            return
        }
        let mut c ;
        while a%b != 0 {
            c = a%b;
            a = b;
            b = c;
        }
        self.numerator = self.numerator/b;
        self.denominator = self.denominator/b;
    }
    pub fn new(x:u64, y:u64, l:bool) -> Fraction{
        let mut c = Fraction{
            numerator:x,
            denominator:y,
            positive:l,
        };
        c.gcd();
        return c
    }
    pub fn from_i32(n:i32)->Fraction{
        Fraction::new(n.abs() as u64, 1, n > 0)
    }
    pub fn from_str(str:&str)->Option<Fraction>{
        let mut x;
        let positive:bool;
        let mut numerator:u64 = 0;
        let mut denominator:u64 = 0;
        let mut s = str.char_indices();
        match s.next(){
            Some(k) =>{
                x = k.1;
            },
            None => {
                return None
            }
        }
        positive = x !=  '-';
        if !positive {
            match s.next(){
                Some(k) =>{
                    x = k.1;
                },
                None => {
                    return None
                }
            }};
        while x.is_ascii_digit() {
            numerator *= 10;
            numerator += x.to_digit(10).unwrap() as u64;
            match s.next(){
                Some(k) =>{
                    x = k.1;
                },
                None => {
                    x = 'c';
                }
            }
        }
        if x == '/'{
            match s.next(){
                Some(k) =>{
                    x = k.1;
                },
                None => {
                    return None
                }
            }
            while x.is_ascii_digit() {
                denominator *= 10;
                denominator += x.to_digit(10).unwrap() as u64;
                match s.next(){
                    Some(k) =>{
                        x = k.1;
                    },
                    None => {
                        x = 'c';
                    }
                }
            }
        }
        Some(Fraction::new(numerator, denominator, positive))
    }
}



impl PartialEq for Fraction{
    fn eq(&self, other: &Self) -> bool{
        if self.numerator == 0 && other.numerator == 0 {return true;}
        else{ return self.numerator == other.numerator && self.denominator == other.denominator && self.positive == self.positive}
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        if self == other {return Some(Ordering::Equal)}
        else if self.numerator*other.denominator > self.denominator*other.numerator{
            return Some(Ordering::Greater)
        }
        else {
            return Some(Ordering::Less)
        }
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }
    
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }
}

impl Display for Fraction {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
         write!(f, "{}{}/{}, num~{}{}", if self.positive{""} else {"-"},
          self.numerator, self.denominator, if self.positive{""} else {"-"},
          self.numerator as f64/self.denominator as f64)
       }
    }

impl Debug for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}/{}", if self.positive{""} else {"-"},
            self.numerator, self.denominator)
            }
}


impl Add for Fraction {
    type Output = Self;
    fn add(self, rhs:Self) -> Self::Output{
        if self.positive != rhs.positive {
            let numerator;
            let positive;
            match self > rhs {
                true =>{
                    numerator = self.numerator*rhs.denominator - self.denominator*rhs.numerator;
                    positive = self.positive;
                }
                false =>{
                    numerator =  self.denominator*rhs.numerator - self.numerator*rhs.denominator;
                    positive = rhs.positive;
                }
            }
            let denominator = self.denominator*rhs.denominator;
            Fraction::new(numerator, denominator, positive)
        }
        else{
        let numerator = self.numerator*rhs.denominator + self.denominator*rhs.numerator;
        let denominator = self.denominator*rhs.denominator;
        Fraction::new(numerator, denominator, self.positive && rhs.positive)
        }
    } 
}
impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs:Self) -> Self::Output{
        let numerator = self.numerator*rhs.numerator;
        let denominator = self.denominator*rhs.denominator;
        Fraction::new(numerator, denominator, !(self.positive ^ rhs.positive))
    } 
}
impl Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs:Self) -> Self::Output{
        let mut x = rhs;
        x.positive = !x.positive;
        self + x
    } 
}
impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{
        let numerator = self.numerator*rhs.denominator;
        let denominator = self.denominator*rhs.numerator;
        Fraction::new(numerator, denominator, !(self.positive ^ rhs.positive))
        } 
    }




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Fraction::new(1, 10, true);
        let b = Fraction::new(2, 10, false);
        let c = a+b;
        let d = c*b+a-c/b;
        let e = Fraction::new(29, 50, true);
        assert_eq!(d, e);
    }
}
