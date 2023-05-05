# rusttd4

TD4 emulator created by Rust language.

TD4 is a simple CPU created by [CPUの創り方](https://www.amazon.co.jp/CPU%E3%81%AE%E5%89%B5%E3%82%8A%E3%81%8B%E3%81%9F-%E6%B8%A1%E6%B3%A2-%E9%83%81/dp/4839909865).


## Demo

![demo](https://user-images.githubusercontent.com/97239922/236500086-c1737130-a5d5-4b79-83a5-0f811e5d34db.gif)

## Requirement

- rustc 1.66.1
- cargo 1.66.1

## Usage

### build binary

```bash
# build application
cargo build
```

### compile asembler source code

```bash
# for ex. compile 'example/asm/led.td4asm'
./target/debug/rusttd4 compile --src ./example/asm/led.td4asm --dst ./example/bin/led
```

### execute compiled binary on TD4 emulator

```bash
./target/debug/rusttd4 emulate --program ./example/bin/led
```

## License

"rusttd4" is under [MIT license](https://en.wikipedia.org/wiki/MIT_License).
