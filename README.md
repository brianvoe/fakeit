# fakeit

[![Latest Version](https://img.shields.io/crates/v/fakeit.svg)](https://crates.io/crates/fakeit)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

Port of the famous [Go fakeit](https://github.com/brianvoe/gofakeit) library for Rust with more than 130 functions.

### Usage

- [Crates.io/fakeit](https://crates.io/crates/fakeit)
- [docs.rs](https://docs.rs/fakeit)

### Functions 

- [address](#address-16-functions)
- [animal](#animal-6-functions)
- [beer](#beer-8-functions)
- [bool](#bool-1-functions)
- [color](#color-4-functions)
- [company](#company-4-functions)
- [contact](#contact-4-functions)
- [currency](#currency-4-functions)
- [datetime](#datetime-14-functions)
- [file](#file-2-functions)
- [generator](#generator-1-function)
- [hacker](#hacker-6-functions)
- [hipster](#hipster-3-functions)
- [image](#image-1-function)
- [internet](#internet-7-functions)
- [job](#job-4-functions)
- [language](#language-3-functions)
- [log-level](#log-level-3-functions)
- [name](#name-5-functions)
- [password](#password-1-function)
- [payment](#payment-6-functions)
- [person](#person-3-functions)
- [status code](#status-code-2-functions)
- [unique](#unique-2-functions)
- [user agent](#user-agent-8-functions)
- [vehicle](#vehicle-6-functions)
- [words](#words-6-functions)

##### address (16 functions)

```rust
extern crate fakeit;

use fakeit::address;

fn main() {
    address::info(); // address::Info struct
    address::street(); // street: 1128 South North Dakota borough
    address::street_number(); // street_number: 3155
    address::street_prefix(); // street_prefix: Port
    address::street_name(); // street_name: Kansas
    address::street_suffix(); // street_suffix: mouth
    address::city(); // city: Schmelerburgh
    address::state(); // state: Kentucky
    address::state_abr(); // state_abr: WA
    address::zip(); // zip: 75221
    address::country(); // country: Romania
    address::country_abr(); // country_abr: BI
    address::latitude(); // latitude: -69.14192
    address::latitude_in_range(-30 as f64, 30 as f64); // latitude_in_range: -18.35571
    address::longitude(); // longitude: 113.12952
    address::longitude_in_range(-30 as f64, 30 as f64); // longitude_in_range: -16.484156
}
```

##### animal (6 functions)

```rust
extern crate fakeit;

use fakeit::animal;

fn main() {
    animal::pet_name(); // pet_name: Squeakers
    animal::animal(); // animal: salmon
    animal::type_of(); // type_of: fish
    animal::farm(); // farm: Sheep
    animal::cat(); // cat: Oriental Shorthair
    animal::dog(); // dog: Rottweiler
}
```

##### beer (8 functions)

```rust
extern crate fakeit;

use fakeit::beer;

fn main() {
    beer::name(); // name: Sierra Nevada Bigfoot Barleywine Style Ale
    beer::style(); // style: Porter
    beer::hop(); // hop: Equinox
    beer::yeast(); // yeast: 1084 - Irish Ale
    beer::malt(); // malt: Roasted barley
    beer::ibu(); // ibu: 75 IBU
    beer::alcohol(); // alcohol: 2.943696 %
    beer::blg(); // blg: 7.4607124°Blg
}
```

##### bool (1 functions)

```rust
extern crate fakeit;

use fakeit::bool;

fn main() {
    bool::bool(); // true / false
}
```

##### color (4 functions)

```rust
extern crate fakeit;

use fakeit::beer;

fn main() {
    color::full(); // full: LightYellow
    color::hex(); // hex: #662461
    color::safe(); // safe: black
    color::rgb(); // rgb: [162, 98, 22]
}
```

##### company (4 functions)

```rust
extern crate fakeit;

use fakeit::company;

fn main() {
    company::company(); // company: Rowe-Schoen
    company::company_suffix(); // company_suffix: Inc
    company::buzzword(); // buzzword: systemic
    company::bs(); // bs: strategic
}
```

##### contact (4 functions)

```rust
extern crate fakeit;

use fakeit::contact;

fn main() {
    contact::info(); // contect::Info
    contact::phone(); // phone: 5173757868
    contact::phone_formatted(); // phone_formatted: 382.450.6544
    contact::email(); // email: benkuvalis@marks.org
}
```

##### currency (4 functions)

```rust
extern crate fakeit;

use fakeit::currency;

fn main() {
    currency::compact(); // currency::Info
    currency::short(); // short: SRD
    currency::long(); // long: Burundi Franc
    currency::price(1 as f64, 123 as f64); // price: 53.7
}
```

##### datetime (14 functions)

```rust
extern crate fakeit;

use fakeit::datetime;

fn main() {
    datetime::month(); // month: 10
    datetime::day(); // day: 10
    datetime::week_day(); // week_day: 6
    datetime::year(); // year: 1986
    datetime::hour(); // hour: 10
    datetime::minute(); // minute: 10
    datetime::second(); // second: 10
    datetime::nanosecond(); // nanosecond: 959678991
    datetime::timezone(); // timezone: SA Pacific Standard Time
    datetime::timezone_full(); // timezone_full: (UTC-04:00) Atlantic Time (Canada)
    datetime::timezone_abv(); // timezone_abv: BST
    datetime::timezone_offset(); // timezone_offset: 13
    datetime::date_range("RFC3339", "RFC3339"); // date_range: 1979-01-06 23:03:10.918301212 UTC
    datetime::date(); // date: 1979-01-06 23:03:10.918301212 UTC
}
```

##### file (2 functions)

```rust
extern crate fakeit;

use fakeit::file;

fn main() {
    file::mime_type(); // mime_type: text/x-fortran
    file::extension(); // extension: aspx
}
```

##### generator (1 function)

```rust
extern crate fakeit;

use fakeit::generator;

fn main() {
    generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string()); // data: Watson Connelly baileeprosacco@smitham.biz 6d0e0a
    // More details about this later
}
```

##### hacker (6 functions)

```rust
extern crate fakeit;

use fakeit::hacker;

fn main() {
    hacker::phrase(); // phrase: parsing the sensor won't do anything, we need to bypass the open-source AGP sensor!
    hacker::abbreviation(); // abbreviation: PCI
    hacker::adjective(); // adjective: bluetooth
    hacker::noun(); // noun: protocol
    hacker::verb(); // verb: copy
    hacker::ingverb(); // ingverb: transmitting
}
```

##### hipster (3 functions)

```rust
extern crate fakeit;

use fakeit::hipster;

fn main() {
    hipster::word(); // word: fingerstache
    hipster::sentence(12); // sentence: Itaque aliquid id ex repudiandae adipisci quibusdam excepturi deleniti qui alias animi.
    hipster::paragraph(3, 4, 40, " ".to_string()); // paragraph: Voluptas minima delectus voluptatibus earum rerum accusamus consequatur sunt....
}
```

##### image (1 function)

```rust
extern crate fakeit;

use fakeit::image;

fn main() {
    image::url(500, 500); // url: https://picsum.photos/500/500
}
```

##### internet (7 functions)

```rust
extern crate fakeit;

use fakeit::internet;

fn main() {
    internet::domain_name(); // domain_name: productvisualize.net
    internet::http_method(); // http_method: DELETE
    internet::domain_suffix(); // domain_suffix: biz
    internet::ipv4_address(); // ipv4_address: 196.140.182.201
    internet::ipv6_address(); // ipv6_address: 2001:cafe:1248:1dc7:17dd:19f4:8798:621d
    internet::mac_address(); // mac_address: 2D:3F:7E:5D:61:C1
    internet::username(); // username: Nienow1881
}
```

##### job (4 functions)

```rust
extern crate fakeit;

use fakeit::job;

fn main() {
    job::info(); // job::Info
    job::title(); // title: Executive
    job::descriptor(); // descriptor: International
    job::level(); // level: Solutions
}
```

##### language (3 functions)

```rust
extern crate fakeit;

use fakeit::language;

fn main() {
    language::random(); // random: Tatar
    language::abbreviation(); // abbreviation: co
    language::programming(); // programming: Rust
}
```

##### log-level (3 functions)

```rust
extern crate fakeit;

use fakeit::log_level;

fn main() {
    log_level::general(); // general: info
    log_level::syslog(); // syslog: crit
    log_level::apache(); // apache: debug
}
```

##### name (5 functions)

```rust
extern crate fakeit;

use fakeit::name;

fn main() {
    name::full(); // full: Keyshawn Auer
    name::first(); // first: Brycen
    name::last(); // last: Hartmann
    name::prefix(); // prefix: Mr.
    name::suffix(); // suffix: PhD
}
```

##### password (1 function)

```rust
extern crate fakeit;

use fakeit::password;

fn main() {
    password::generate(upper, numeric, special, num); // #9e1Vv5s&Ng8L-#9@=!6+s1+0@R
}
```

##### payment (6 functions)

```rust
extern crate fakeit;

use fakeit::payment;

fn main() {
    payment::credit_card(); // payment::CreditCard
    payment::credit_card_type(); // credit_card_type: Discover
    payment::credit_card_number(); // credit_card_number: 341545247171534
    payment::credit_card_luhn_number(); // @TODO
    payment::credit_card_exp(); // credit_card_exp: 04/21
    payment::credit_card_cvv(); // credit_card_cvv: 537
}
```

##### person (3 functions)

```rust
extern crate fakeit;

use fakeit::person;

fn main() {
    person::info(); // person::Info
    person::ssn(); // ssn: 792671651
    person::gender(); // gender: male
}
```

##### status code (2 functions)

```rust
extern crate fakeit;

use fakeit::status_code;

fn main() {
    status_code::simple(); // simple: 404
    status_code::general(); // general: 400
}
```

##### unique (2 functions)

```rust
extern crate fakeit;

use fakeit::unique;

fn main() {
    unique::uuid_v1(); // uuid_v1: 13be40a6-1dd2-11b2-802a-010203040506
    unique::uuid_v4(); // uuid_v4: a474961e-936a-4897-966a-15fcff7bbc87
}
```

##### user agent (8 functions)

```rust
extern crate fakeit;

use fakeit::unique;

fn main() {
    user_agent::chrome(); // chrome: Mozilla/5.0 (X11; Linux i686) AppleWebKit/532 (KHTML, like Gecko) Chrome/36.0.861.0 Mobile Safari/532
    user_agent::firefox(); // firefox: Mozilla/5.0 (X11; Linux x86_64; rv:7.0) Gecko/2005-5-27 Firefox/36.0
    user_agent::safari(); // safari: Mozilla/5.0 (Windows; U; Windows NT 6.2) AppleWebKit/531.23.3 (KHTML, like Gecko) Version/4.0 Safari/531.23.3
    user_agent::opera(); // opera: Opera/8.22 (Macintosh; PPC Mac OS X 10_6_8; en-US) Presto/2.11.181 Version/12.00
    user_agent::linux_platform_token(); // linux_platform_token: X11; Linux x86_64
    user_agent::mac_platform_token(); // mac_platform_token: Macintosh; U; PPC Mac OS X 10_6_2
    user_agent::windows_platform_token(); // windows_platform_token: Windows 98; Win 9x 4.90
    user_agent::random_platform(); // random_platform: Macintosh; Intel Mac OS X 10_7_5
}
```

##### vehicle (6 functions)

```rust
extern crate fakeit;

use fakeit::vehicle;

fn main() {
    vehicle::info(); // vehicle::Info
    vehicle::vehicle_type(); // vehicle_type: Passenger car mini
    vehicle::fuel(); // fuel: Electric
    vehicle::transmission_gear(); // transmission_gear: Automatic
    vehicle::car_maker(); // car_maker: Chevrolet
    vehicle::car_model(); // car_model: Gti
}
```

##### words (6 functions)

```rust
extern crate fakeit;

use fakeit::words;

fn main() {
    words::word(); // word: saepe
    words::sentence(word_count); // sentence: Nemo vitae rerum consequuntur vero animi incidunt esse doloribus eos.
    words::paragraph(count, sentence_count, word_count, separator); // paragraph: Minima aut numquam nihil rerum commodi pariatur dolores...
    words::question(); // question: Placeat voluptatem at ut eveniet suscipit similique dicta quis?
    words::quote(); // quote: "Dignissimos dolorem quam tempore excepturi facere dicta." - Willy Kihn
    
    let opts = words::ParagraphOpts {
        count: 5,
        sentence_count: 4,
        word_count: 11,
        separator: "\n".to_string(),
    };
    words::paragraph_generator(opts, &words::sentence); // paragraph_generator: Quisquam aut consequuntur nobis voluptas porro...
}
```
