//This is about lifetimes in rust.
//Lifetime Elision Rules:
// 1. Each input prameter that is a reference is assigned its own lifetime
//      fn foo <'a>(x:&'a str, y:i32) -> & str{}
// 2. If there is exactly one input lifetime, assign it to all output lifetimes
//      fn foo <'a>(x:&'a str) -> &'a str{}
// 3. if there is a &self or &mut self input parameter, its lifetime will be assigned to all output
//    lifetimes.
fn main() {}
