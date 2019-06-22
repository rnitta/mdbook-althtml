# mdbook-althtml
[![Build Status](https://travis-ci.org/rnitta/mdbook-althtml.svg?branch=master)](https://travis-ci.org/rnitta/mdbook-althtml)  

***Now under development.***

An alternative of the official [mdbook](https://github.com/rust-lang-nursery/mdBook) html renderer.  

Though mdbook is a highly modifiable tool, but you should develop your own renderer when you want to make some changes to the process of rendering html.
Developing renderer from scratch is not easy. So, we provide more modifiable renderer and you can some (partial) changes with few code.

For now, only post-processors can be added. 

# todo: Docs
https://rnitta.github.io/mdbook-althtml/

# todo: Usage
You can use this as well as official html renderer.

```rust
some code here
```

For more details, see `example/`.

# Todo
- Write and build docs.
- Modifiable handlebars "in rendering templates."
- Modifiable markdown converter.

# Disclaimer
The main target is only to provide more modifiable html renderer for mdbook, 
so the compatibility to the mdbook html renderer (not "to mdbook") may well not be guaranteed.

Great part of this crate's code are copy-and-pasted from official mdbook.  

# Contributing
Issue/PRs are welcome
