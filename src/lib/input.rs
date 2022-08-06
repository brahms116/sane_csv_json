use super::*;
use serde_json::Value;

/// User arguments passed in to the program
pub struct InputArgs<'a> {
    /// path of the csv file to take as input
    pub csv_path: &'a str,
    /// path of json file to output
    pub out_path: Option<&'a str>,
    /// path of json file containing the configuration
    pub config_path: Option<&'a str>,
}

/// Formatted inputs to our application
pub struct FormattedInputs {
    /// complete schema set generated from defaults and user input
    pub schema: Vec<ColumnDef>,
    /// records extracted from csv file
    pub records: Vec<Vec<String>>,
    /// The desired output path of the resulting json file
    pub output_path: String,
}

/// Utilities for handling file and user inputs
///
/// Handles reading config and csv files and also providing defaults
/// where neccesary
pub struct InputService();

type JsonConfig = Vec<ColumnDefOptions>;

impl InputService {
    /// Takes in user inputs and files and returns column definitions and csv data
    pub fn handle_input(n: InputArgs) -> Result<FormattedInputs> {
        /* Parse the csv records */
        let rdr = Reader::from_path(n.csv_path).map_err(|_| {
            WhoopsBuilder::new()
                .context("whilst trying to read csv data")
                .why("file not found")
                .err_type("csv-file-not-found")
                .suggestion(
                    "specify a valid path to the csv file you want to convert via the -d option",
                )
                .build()
        });

        /* reject if failed to parse records */
        if let Err(err) = rdr {
            return Err(err);
        }

        let mut rdr = rdr.unwrap();

        let headers = rdr.headers();
        if let Err(err) = headers {
            return Err(WhoopsBuilder::new()
                .err_type("failed-parse-csv-header")
                .context("whilst trying to parse csv headers")
                .why(&format!("{}", err))
                .suggestion("ensure that csv headers are valid")
                .build());
        }

        let headers: Vec<String> = headers
            .unwrap()
            .clone()
            .into_iter()
            .map(|e| e.to_owned())
            .collect();
        let header_len = headers.len();

        let records = rdr.records();

        let mut string_records: Vec<Vec<String>> = vec![];
        for (i, data) in records.into_iter().enumerate() {
            if let Err(err) = data.as_ref() {
                error!(
                    "{}",
                    WhoopsBuilder::new()
                        .err_type("failed-parse-row")
                        .context(&format!("parsing csv at row {}", i + 1))
                        .why(&format!("{}", err))
                        .suggestion("ensure csv row is valid")
                        .build()
                )
            }

            let data = data.unwrap();
            let mut vec: Vec<String> = Vec::new();
            vec.reserve(header_len);

            for n in data.into_iter() {
                vec.push(n.to_owned());
            }
            string_records.push(vec);
        }

        /* handle configs */
        let mut config: JsonConfig = vec![];

        /* If there isn't a config path, we will generate a default config */
        if let None = n.config_path {
            error!(
                "{} will proceed with generating default options",
                WhoopsBuilder::new()
                    .err_type("missing-config-path")
                    .context("whilst looking for the config json")
                    .why("file not specified")
                    .suggestion("specify the config file with the -c option")
                    .build()
            );
        } else {
            let file = std::fs::read_to_string(n.config_path.unwrap());

            if let Err(err) = file {
                return Err(WhoopsBuilder::new()
                    .err_type("failed-to-open-config")
                    .context("whilst tyring to read the config file")
                    .why(&format!("{}", err))
                    .suggestion("ensure the specified file is valid")
                    .build());
            }

            let file = file.unwrap();
            let read_res = serde_json::from_str::<JsonConfig>(&file);

            if let Err(err) = read_res {
                error!(
                    "{} will proceed with default configs",
                    WhoopsBuilder::new()
                        .err_type("failed-to-parse-config")
                        .context("whilst tyring to parse the config file")
                        .why(&format!("{}", err))
                        .suggestion("ensure the config syntax is correct")
                        .build()
                );
            } else {
                config = read_res.unwrap();
            }
        }

        /* if there isn't an output path, we will call it data.json */
        let mut output_path = "data.json".to_owned();

        if let Some(path) = n.out_path {
            output_path = path.to_owned();
        }

        let schema = Self::fill_schema_defaults(config, headers, header_len);

        Ok(FormattedInputs {
            schema,
            records: string_records,
            output_path,
        })
    }

    /// Fills completes user schema configuration with defaults and returns column definitions
    fn fill_schema_defaults(
        mut input_config: Vec<ColumnDefOptions>,
        headers: Vec<String>,
        required_len: usize,
    ) -> Vec<ColumnDef> {
        let config_len = input_config.len();
        if config_len < required_len {
            error!(
                "{} will fill remaining schemas with defaults",
                WhoopsBuilder::new()
                    .err_type("wrong-schema-length")
                    .context("while checking schema")
                    .why("schema length is shorter than num of columns provided in the csv")
                    .suggestion("ensure that the schema is of the correct length")
                    .build()
            );

            for _i in 0..required_len - config_len {
                input_config.push(ColumnDefOptions::default());
            }
        } else if config_len > required_len {
            error!(
                "{} will ignore extra schemas",
                WhoopsBuilder::new()
                    .err_type("wrong-schema-length")
                    .context("while checking schema")
                    .why("schema length is longer than num of columns provided in the csv")
                    .suggestion("ensure that the schema is of the correct length")
                    .build()
            );

            input_config.truncate(required_len);
        }

        let mut res: Vec<ColumnDef> = vec![];

        for (i, col) in input_config.into_iter().enumerate() {
            let mut name = i.to_string();
            if let Some(n) = col.col_name {
                name = n
            } else if let Some(n) = headers.get(i) {
                info!("No name given for column {}, using csv name {}", i, n);
                name = n.to_string()
            } else {
                error!(
                    "{} will leave name as {}",
                    WhoopsBuilder::new()
                        .err_type("col-no-name")
                        .context("while checking schema")
                        .why(&format!("column {} has no name", name))
                        .suggestion("ensure that column is given a name either through the csv or through the schema config")
                        .build(),
                    name
                );
            }

            let mut col_type = ColType::String;
            if let Some(n) = col.col_type {
                col_type = n
            } else {
                info!("No type given for {}, will default to String", name)
            }

            let mut default = Value::Null;
            if let Some(n) = col.col_default {
                default = n
            } else {
                info!("No default for {}, will default to Null", name)
            }

            let mut format = "%d/%m/%Y".to_owned();

            if let ColType::Date = col_type {
                if let Some(n) = col.col_format {
                    format = n
                } else {
                    error!(
                    "{} will leave format as {}",
                    WhoopsBuilder::new()
                        .err_type("date-no-format")
                        .context("while checking schema")
                        .why(&format!("column {} is a date column but no format was given", name))
                        .suggestion("ensure that date columns have a specified format via the format key")
                        .build(),
                        format
                        )
                }
            }

            let mut true_string = "true".to_owned();
            let mut false_string = "false".to_owned();
            if let ColType::Bool = col_type {
                if let Some(n) = col.col_true_string {
                    true_string = n;
                } else {
                    info!(
                        "column {} was not given true-string, will default to \"true\"",
                        name
                    );
                }

                if let Some(n) = col.col_false_string {
                    false_string = n;
                } else {
                    info!(
                        "column {} was not given false-string, will default to \"false\"",
                        name
                    );
                }
            }

            res.push(ColumnDef {
                col_type,
                col_name: name,
                col_format: format,
                col_true_string: true_string,
                col_false_string: false_string,
                col_default: default,
            })
        }
        res
    }
}
