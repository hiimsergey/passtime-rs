# passtime-rs
A lil' Rust program that output randomly generated passwords according to your flags.

Then, you can redirect the output into a file or pipe it.

```
A CLI-based password generator

Usage: passtime [OPTIONS] <LENGTH>

Arguments:
  <LENGTH>  Password length

Options:
  -a                   Include lowercase letters
  -A                   Include capital letters
  -n                   Include numbers
  -c <CHARACTERS>      Include custom characters
  -i <TEXT>            Print text before the password
  -h, --help           Print help
  -V, --version        Print version
```
