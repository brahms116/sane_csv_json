# Sane Csv Json

A configurable cli tool converting csv into json

This is a tool which allows users to pass in configuration via a json file.
Users can rename columns and specify the desired type for each column

This crate was built using...

- [serde](https://github.com/serde-rs/serde)
- [serde_json](https://github.com/serde-rs/json)
- [csv_rs](https://github.com/BurntSushi/rust-csv)
- [chrono](https://github.com/chronotope/chrono)
- [clap](https://github.com/clap-rs/clap)

## Example

```bash
sane_csv_json input.csv --config-path config.json
```

The output path defaults to `data.json` however, it can be configured via the `--ouput-path` flag

## Configuration

Below is an example configuration

```json
[
  {
    "type": "string",
    "name": "name",
    "default": "unnamed"
  },
  {
    "type": "integer",
    "name": "landSize"
  },
  {
    "type": "float",
    "name": "cost"
  },
  {
    "type": "string",
    "name": "type"
  },
  {
    "type": "boolean",
    "name": "isOverseas",
    "trueString": "yes",
    "falseString": "no"
  },
  {
    "type": "date",
    "name": "saleDate",
    "format": "%d/%m/%Y"
  }
]
```

### `type`

specifies the desired type for the column can be one of ....

- `integer`
- `float`
- `date`
- `boolean`

### `name`

refers to the desired name of the column, if left blank, the csv header will be used

### `default`

you can provide a default value with this key, if the parse fails or
data is blank, this default will be used. Defaults to `null`

### `format`

only applicable if `type` is set to `date`, uses chrono's `NaiveDate` at the
moment, see relevant formats
[here](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

### `trueString` & `falseString`

only applicable when `type` is set to `boolean`, specifies the strings which to parse as `true` and `false`
