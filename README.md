# DECIDER

A program written in Rust that will help deciding which file to play from a
multimedia collection, whether be a movie or music.

I wrote it as a way to learn Rust, and it's explained in [my web] (in spanish).

[my web]: https://www.ylabs.es/article/empezando-con-rust/


## Usage

There are four options that **_decider_** can accept:
```
Usage: decider [options]

Options:
    -h, --help          prints this help screen
    -i, --input DIR     defines the input directory
    -c, --count COUNT   how many files should we get?
    -p, --play          should we play the selected file?
```

By default, if we call it without any option, only one file from the current
directory will be shown. The option **p** uses **vlc** as player: take this
into account if you compile the source unmodified. If you want to show more
than one file with option **c**, option **p** will be ignored.


## Issues **not yet** addressed

* **_decider_** will panic if you try to select something from a directory where
files has no extensions.
* When try to show more than one file, sometimes we get duplicate entries.
* It's not very useful yet if we try to select music.
* As it uses no recursion, the way we use to store our files will greatly affect
the results.

## License

**_decider_** is distributed under the terms of the MIT license.
