- [Rust Linz, October 2021 - Rust Closures by Rainer Stropek](https://www.youtube.com/watch?v=bgZa9VRBhY)

Closures captures the environment means, it has ability to read the values of a variable defined with that running context 

Lowercase `fn` means, it is either function or function pointer (this depends on the scenario where it is being used)

Uppercase `Fn` means, it is a **Trait** which does not mutate the captured variables and borrows the variable in the form immutable.

Uppercase `FnMut` means, it is a **Trait** which does mutation of the captured variables or in other words it is a mutable borrowed variable.

Uppercase `FnOnce` means, it is a **Trait** which we can use only once as it is taking ownership