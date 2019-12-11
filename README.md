# Rust error playground

This repository has goal to display various error managements 
for [Rust](https://www.rust-lang.org/) programing language.

The Rust language has a wonderful error management using the `Result` type but it can be tricky to understand how trait 
`Error` is handled and how contextualise errors for our programs, especially for new comer (in which I include myself in).

The main goal is to test libraries and check how you can work and code using all of these.

This repo is my tests and is not accurate, feel free to review and request change in case of misunderstandings or improvment.

## Common problem

The common problem is to read an file containing the Shakespeare Romeo and Juliette piece downloaded on the 
[Guteberg project](https://www.gutenberg.org/), read the file content, remove the legals sections and split it into 
chapters from the file.

## Playground

### Vanilla

[Code](vanilla)

Run : `cargo run --package vanilla --bin vanilla`

### Vanilla with custom errors

[Code](vanillacustom)

Run : `cargo run --package vanillacustom --bin vanillacustom`