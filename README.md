```bash
$ RUSTFLAGS="-Z ui-testing" cargo test --test ui
```

```
error[E0277]: the trait bound `Foo: MyTrait` is not satisfied
  --> tests/ui.rs:7:17
   |
LL |     my_function(foo);
   |                 ^^^ the trait `MyTrait` is not implemented for `Foo`
   |
note: required by a bound in `my_function`
  --> /Users/kevin/scratch/ui-testing/src/lib.rs:3:38
   |
LL | pub fn my_function<I>(_: I) where I: MyTrait {
   |                                      ^^^^^^^ required by this bound in `my_function`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `ui-testing` due to previous error
```
