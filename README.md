cc3166
===
A minimalistic `Country` type for working with Country Codes as defined in 
[ISO 3166-1][1].

Excerpt from [page][1]:
> The country codes can be represented either as a two-letter code (alpha-2) 
> which is recommended as the general-purpose code, a three-letter code 
> (alpha-3) which is more closely related to the country name and a three-digit 
> numeric code (numeric-3) which can be useful if you need to avoid using Latin 
> script.

Using optional features the crate can provide country name translations into
featured language using `in_<language>()`.

Currently provided translations (in lowercase for feature name):
* Chinese
* Finnish
* French
* Russian
* Spanish
* Swedish

Note that the underlying data is derived from open public sources only, as such 
data quality is **not** guaranteed.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cc3166 = { version = "0.1", git = "https://github.com/procrustus/cc3166-rs" }
```


## Example

```rust
use cc3316::Country;

let a3_str = "USA";

match Country::from_a3(a3_str) {
    Ok(code) => {
        // `Display` is implemented, displaying the Alpha-2 code.
        
        // `Debug` is implementented, diplaying the Country enum value.
        println!("{:?}", code); // "Country::US"
        // `alpha2()` returns the recommended general-purpose alpha-2 code.
        println!("  Alpha-2 code: {}", code.alpha2()); // "US"
        // `alpha3()` returns the alpha-3 code which is more closely related to 
        // the country name.
        println!("  Alpha-3 code: {}", code.alpha3()); // "USA"
        // `numeric()` returns the numeric code which can be useful if you need 
        // to avoid using Latin script.
        println!("  Numeric code: {}", code.numeric()); // 840
        // `country_data()` returns a reference to the underlying basic 
        // `CountryData` which includes the Alpha-2/-3 & numeric code.
        //
        // CountryData("US", "USA", 840) for `code`.
        println!("  Country data: {:?}", code.country_data()); 
    }
    Err(err) => panic!("Unable to parse country code `{}`: {}.", a3_str, err),
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[1]: https://www.iso.org/iso-3166-country-codes.html