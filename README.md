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

## Basic Usage

### Implementing

Add line to cargo.toml :
```toml
prtgn_encoding = { git = "https://github.com/PRTGN-Development-Team/PRTGN_encoding" }
```

### Write

```rust
use prtgn_encoding::write;

fn main() {
    
    let filename = "test.prtgn".to_string();
    let text = "Hello world! This is some example text.".to_string();
    
    write(filename, text).unwrap();
}
```

### Read

```rust
use prtgn_encoding::read;

fn main() {
    
    let filename = "test.prtgn".to_string();

    let read_text = read(filename).unwrap().to_string();
}
```
