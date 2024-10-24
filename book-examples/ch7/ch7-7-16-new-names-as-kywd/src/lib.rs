// // use std::fmt;
// // use std::io;
// // fn function1() -> fmt::Result {
// // //  --snip--
// // }
// // fn function2() -> io::Result<()> {
// // //  --snip--
// // }

// // use std::fmt::Result;
// // use std::io::Result as IoResult;

// // nested paths
// use std::{fmt::Result, io::Result as IoResult};

// // use std::io;
// // use std::io::Write;

// // shorthanded
// use std::{self, io::Write};

// // glob operator
// use std::collections::*;    // brings all the public elements into scope

// fn function1() -> Result {
// //  --snip--
//     return ();
// }
// fn function2() -> IoResult<()> {
// //  --snip--
//     return ();
// }