# Commission Records for Artists

[Usage](Usage.md)

Commission Records for Artists, or simply C.R.A., is a command line interface for storing art commissions and YCHs information into CSV files. The information can later be imported into databases or modified externally from a spreadsheet program.

I welcome contribution to this project.

## Requirements

### Prerequisites

- Rust 2018 edition

## Configuration

Art Manager expects the ``config.toml`` in the ``/artm`` directory of your operating system's respective document's folder. If the file isn't found, it will use the default settings.

## Currency

The default currency is USD. This can be overwritten in the ``config.toml`` or ``--currency`` option. You must provide a [ISO-4127 code](https://en.wikipedia.org/wiki/ISO_4217#Active_codes) in either case. The latter is for per-client bases whereas the former is for your region's currency.

```toml
currency = "EUR"
```

When giving a fee amount, you do not need to provide a decimal. The rounding would be done automatically by the program.

## License

I license this project under the GNU GPL v3 - see the [LICENSE](LICENSE) file for details
