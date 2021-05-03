// a phantom type parameter is one that doesn't show up ar runtime, but
// is checked statically (and only) at compile time
//
// in the following example, we combine std::marker::PhantomData with the 
// phantom type parameter concept to create tuples containing different
// data types

use std::marker::PhantomData;

// a phantom tuple struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// a phantom type struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

fn main() {
    // here `f32` and `f64` are the hidden paramters
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("_tuple1_ == _tuple2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
