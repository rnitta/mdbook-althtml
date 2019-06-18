# mdbook-althtml
{{{ marquee_start }}}
mdbook-althtml
{{{ marquee_end }}}

## What's this?
An alternative of the official [mdbook](https://github.com/rust-lang-nursery/mdBook) html renderer.  

Though mdbook is a highly modifiable tool, you should develop your own renderer if you want to make some changes to the way of rendering html.  
Developing renderer from scratch is not easy. So, we provide more modifiable renderer and you can some changes with few code.

For now, only post-processors can be added.
As an example, I developed handlebars post-processor `src/post_processor/hbs_processor.rs`.
