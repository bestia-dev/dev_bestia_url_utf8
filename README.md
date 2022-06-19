[comment]: # (auto_md_to_doc_comments segment start A)

# dev_bestia_url_utf8

[comment]: # (auto_cargo_toml_to_md start)

**Library for url utf8 encode/decode**  
***version: 0.1.28  date: 2021-10-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/dev_bestia_url_utf8)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-135-green.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-79-blue.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-32-purple.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-38-orange.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/)

[comment]: # (auto_lines_of_code end)

[comment]: # (auto_badges start)

[![crates.io](https://img.shields.io/crates/v/dev_bestia_url_utf8.svg)](https://crates.io/crates/dev_bestia_url_utf8)
[![Documentation](https://docs.rs/dev_bestia_url_utf8/badge.svg)](https://docs.rs/dev_bestia_url_utf8/)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_url_utf8.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_url_utf8/)
[![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_url_utf8/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/blob/master/LICENSE)
[![Rust](https://github.com/bestia-dev/dev_bestia_url_utf8/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/dev_bestia_url_utf8/actions)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fdev_bestia_url_utf8&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

[comment]: # (auto_badges end)

Hashtags: #rustlang #url #tutorial
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

Urls must be constantly encoded and decoded.  
It deserves something short like the macro `url!("x/y/{}", name)`.  
This module is strictly limited to utf8 urls.  
Url is made of parts, fragments or segments mostly delimited by slash "/".  
They must be separately encoded/decoded, not as a whole url string.  
It is impossible to guarantee that the whole string is correctly encoded/decoded.  
But is possible to minimize the misuse of the String type for Url.  
With the normal String it is not possible to force the developer to encode/decode.  
With special wrapper types around String is possible to help the coder to write properly and not forget about it.  
TODO: analyze if is possible to use more &str and Cow instead of always allocating String.  
But urls are usually small and this is not a priority.  

## Development

I use [cargo-auto](https://crates.io/crates/cargo-auto) for my automation tasks like `cargo auto build` or `cargo auto doc`, ...

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web:  
<https://web.crev.dev/rust-reviews/crates/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful,  
please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
