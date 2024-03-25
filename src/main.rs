use std::fmt::{Debug,Display,Formatter,Result};
use std::ops::{Add, Div, Mul, Sub};
use my_fraction::Fraction;


type Tensor<const N: usize> = [Fraction;N];
#[derive(Copy, Clone, Debug)]
struct Matrix<const N: usize>{
    tensor:Tensor<N>,
    x:u8,
    y:u8
}

impl<const N: usize> Display for Matrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut n = 0;
        write!(f, "{} X {}  [\n",self.x, self.y)?;
        for _i in 0..self.x{
            write!(f, "[ ")?;
            for _j in 0..self.y{
                let tem = self.tensor[n];
                write!(f, "{}{}/{}, num~{}{:.6}  ", if tem.positive{""} else {"-"},
                tem.numerator, tem.denominator, if tem.positive{""} else {"-"},
                tem.numerator as f64/tem.denominator as f64)?;
                n += 1;
            }
            write!(f, "]\n")?;
        }
        write!(f, "]\n")?;
     Ok(())
   }
}

impl<const N: usize> Matrix<N> {
    fn new(tensor:[Fraction;N], x:u8, y:u8)->Option<Matrix<N>>
    {
        if N == x as usize * y as usize {
            Some(
                Matrix{
                    tensor:tensor,
                    x:x,
                    y:y
        })
        }
        else{
            None
        }
    }
    fn from_i32(ten:[i32;N],x:u8,y:u8)->Option<Matrix<N>>{
        if N == x as usize * y as usize {
            let mut tensor:[Fraction;N] = [Fraction::from_i32(0);N];
            for i in 0..N{
                tensor[i] = Fraction::from_i32(ten[i]);
            }
            Some(
                Matrix{
                    tensor:tensor,
                    x:x,
                    y:y
                }
            )
        }
        else{
            None
        }
    }
    fn new_same(val:i32, x:u8, y:u8)->Box<Option<Matrix<N>>>
    {
        Box::new(Matrix::new([Fraction::from_i32(val);N],x,y))
    }
}

impl<const N: usize> Add for Matrix<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output{
        let mut re = self.clone();
        for i in 0..N{
            re.tensor[i]  = re.tensor[i] + rhs.tensor[i];
        }
        re
    }
}

impl<const N: usize> Sub for Matrix<N>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output{
        let mut re :Matrix<N> = self.clone();
        for i in 0..N{
            re.tensor[i]  = re.tensor[i] - rhs.tensor[i];
        }
        re
    }
}

impl<const N: usize> Mul for Matrix<N>{
    type Output = Box<Option<Matrix<N>>>;
    fn mul(self, rhs: Self) -> Self::Output{
        if self.y == rhs.x{
            let mut re = Matrix::new_same(0,self.x,rhs.y).unwrap();
            let n = self.x*rhs.y;
            for i in 0..n{
                re.tensor[i] = 
            }
            Box::new(Some(re))
        }
        else{
            Box::new(Option::None)
        }
    }
}






fn main() {
    let x = Box::new(Matrix::new([Fraction::from_i32(30);10],1,10));
    println!("{:?}",x.unwrap().tensor[1])
}
