<h1 align="center">
  rustpass
</h1>
<h2 align="center">
  
  My [Password Generator](https://github.com/j0giwa/password-generator), rewritten in Rust
  
</h2>
<p align=center>
  <br>
  <img src="https://img.shields.io/badge/os-linux-brightgreen">
  <img src="https://img.shields.io/badge/os-mac-brightgreen">
</p>

## Table of Contents

- [Install](#Install)
- [Usage](#Usage)
- [Disclaimer](#Disclaimer)

## Install

TDL

## Usage

```sh
rustpass
```

will output a random String with the default parameters

Optional flag can be suplied.

| flag               | function                           |
| ------------------ | ---------------------------------- |
| -l <amount>        | specify the amount of characters   |
| --lenth <amount>   | specify the amount of characters   |

example:

```sh
rustpass -l 12
```

will output a 12 character string.

