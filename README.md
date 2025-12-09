# PRTGN Encoding

A Rust library for encoding PRTGN files. 


> [!CAUTION]
> 
>PRTGN Encoding, a Rust library for encoding PRTGN files. For use in programs relating to https://github.com/PRTGN-Development-Team/.prtgn
>Copyright (C) 2025 PRTGN Development Team
>
>This program is free software: you can redistribute it and/or modify
>it under the terms of the GNU General Public License as published by
>the Free Software Foundation, either version 3 of the License, or
>(at your option) any later version.
>
>This program is distributed in the hope that it will be useful,
>but WITHOUT ANY WARRANTY; without even the implied warranty of
>MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
>GNU General Public License for more details.
>
>You should have received a copy of the GNU General Public License
>along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Overview

`PRTGN_encoding` (listed as `prtgn_encoding` for Cargo.TOML) is a library used by programs compatible with PRTGN files to encode and decode them.

Any file can be encoded with PRTGN, though for user convenience it is highly recommended to use the .prtgn extension. What is being written isn't a text file, simply add the original file extension to the end. Such as .prtgn_wav does.

Going along with that, anything can be encoded with PRTGN. As long as what is given to the `write` function as a string.
     An example of this is used by PRTGN for the wav file. [wav_converter.rs | PRTGN version 0.3.1, added in version 0.3.0](https://github.com/PRTGN-Development-Team/.prtgn/blob/83d6a200cdf14e82b84684480198a63ae40c63da/src/command/prtgn_wav/wav_converter.rs).
     `wav_converter.rs` takes a wav file, reads its data, converts the data buffer to a string, and then writes it to a `prtgn_wav` file. The opposite is done for playing the wav file.

Simply add PRTGN_encoding to your Cargo.TOML, and add the following to your file to access commands depending on what you need!

## Read Only
```Rust
use prtgn_encoding::read;
```

## Write Only
```Rust
use prtgn_encoding::write;
```

## Read and Write
```Rust
use prtgn_encoding::{read, write};
```

---


# Example File Usage


## Basic Write Example
```commandline
cargo run --example basic_write
```

## Basic Read Example

**Run the write example first in order to have a correctly encoded PRTGN file with the right name**

```commandline
cargo run --example basic_read
```

## Read Write Combo Example
```commandline
cargo run --example read_write
```
