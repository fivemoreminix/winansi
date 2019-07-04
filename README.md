# winansi
Enable ANSI support for a Windows console program.

After reading more, it sounds like this is done automatically by cmd.exe, but this should work for any other rare circumstances; for example... not being enabled automatically.

I wrote this on Linux, so I am unable to test the code, but it *could* work. If it doesn't work, please correct the code that's there (should be a syntactic issue) and make a pull request. Many thanks!

## Usage
Add this to your dependencies section:
```toml
winansi = { git = "https://github.com/asmoaesl/winansi" }
```
*Until I know this works and can actually add it to Crates.io.*

Then use it like this:
```rust
use winansi::windows_ansi;

fn main() {
    windows_ansi();

    // now do all your ANSI stuff :)
}
```
