## grepru
This is a clone of the popular ```grep``` linux command written in rust.

![Rust](https://github.com/AlexJuca/grepru/workflows/Rust/badge.svg)

#### Description

`grepru` searches for `PATTERNS` in each `FILE`.
`grepru` is used in a shell command.


### Current supported functionality:

#### Search for string in file
```bash
grepru "if the lady" file.txt

```

#### Print version
```bash
grepru --version || grepru --V

```

#### Print only a count of selected lines per FILE
```bash
grepru "hello" file.txt -c
```

License
----------------

The library is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
