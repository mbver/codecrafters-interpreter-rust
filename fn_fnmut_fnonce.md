Fn:FnMut:FnOnce

- Fn can only read the capture values
- FnMut can mutate the captured values
- FnOnce can consume the captured values

If f is Fn and g accepts FnMut as input, we can pass f to g:

- here, g doesn't have to mutate f's closure. it only accepts a function that CAN mutate its closure. the function can mutate its OWN closure inside g. but if the function doesn't mutate its own closure, g is ok with that.

```
let x = 10;
let f = || println!("{}", x);

fn g(mut f: impl FnMut()) {
  f()
}
```

- however, if we pass h() that consume its own closure to g, g will not accept that. g only allows the function to mutate its closure or read it, not consume it.

- when we define a closure, the compiler will generate code for the it and using Fn, FnMut, FnOnce depends on how it deal with captured value
