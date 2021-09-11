pub trait MyTrait {}

pub fn my_function<I>(_: I) where I: MyTrait {
    todo!()
}
