[![Winnipeg API Client Test](https://github.com/MaFeLP/computerscience402-final-project/actions/workflows/transit-api-client.yml/badge.svg)](https://github.com/MaFeLP/computerscience402-final-project/actions/workflows/transit-api-client.yml)

# transit-api-client

A rust client for the Winnipeg Transit API version 3, built with reqwest.

In order to run the tests, the environement variable `WPG_TRANSIT_API_KEY` must be set
to the API key. An API Key can be optained at <https://api.winnipegtransit.com/>.

## Testing
Unit-Testing is done with cargo:

```bash
cargo test
```

## Documentation
Generate only the documentation for this library and not its dependencies:

```bash
cargo doc --offline --no-deps
```

## Linting
Linting is done with cargo's built-in linter, as well as clippy:

```bash
cargo fmt -- --check
cargo clippy --all-features --no-deps
```

