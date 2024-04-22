use num::complex::Complex;

trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

impl<T: PartialEq + Eq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

fn main() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 5, im: 2 };

    assert_eq!(x == y, x.eq(&y));
    println!("{:?}", x.eq(&y));
}
