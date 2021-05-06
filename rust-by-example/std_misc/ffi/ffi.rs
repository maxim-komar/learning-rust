// Foreign functions must be declared inside an `extern` block annotated
// with a `#[link]` attribute containing the name of the foreign
use std::fmt;

// this extern block links to the libm library
#[link(name = "m")]
extern {
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 0. };

    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    println!("cos({:?}) = {:?}", z, cos(z));
}

// minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
