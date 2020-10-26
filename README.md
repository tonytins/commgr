# Art Manager

[Usage](Usage.md)

A command line interface for storing art request, commission, and YCH information. The application is a work in progress but contribution is welcomed.

## Requirements

### Prerequisites

- Rust 2018 update

## Configuration

Art Manager expects the ``config.toml`` in the ``/artm`` directory of your operating system's respective document's folder. If the file isn't found, it will use the default settings.

```toml
# The default is USD
currency = "EUR"
```

## License

This project is licensed under the GNU GPL v3 License - see the [LICENSE](LICENSE) file for details
