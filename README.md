# mab-template-namada

A template to generate [`mab`](https://codeberg.org/sugo/mab)
[Namada](https://github.com/anoma/namada) debugging code from.

## Template usage

To use this template, install
[`cargo-generate`](https://github.com/cargo-generate/cargo-generate), and run:

```
cargo generate --git https://github.com/heliaxdev/mab-template-namada
```

## Interaction with mab

To use this template with `mab`, simple run `cargo build --release`, and pass
the generated `libmab_template_rust` library to the `MAB_LIB_PATH` environment
variable.

## License

This code is licensed to the public domain under CC0.

You can find a copy of the legal text
[here](https://creativecommons.org/publicdomain/zero/1.0/legalcode).
