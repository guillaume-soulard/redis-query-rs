use redis::Value;
use std::io::{stderr, stdout, Write};

const DEFAULT_OUTPUT_FORMAT: &str = "{stdout}";
const DEFAULT_STRING_OUTPUT_FORMAT: &str = "\"{stdout}\"";
const DEFAULT_ERROR_OUTPUT_FORMAT: &str = "(error) ERR \"{stdout}\"";
const DEFAULT_INPUT_VALUE: &str = "";


pub fn writeln_redis_value_to_stdout_for_cli(value: Value) {
    match value {
        Value::Nil => writeln_to_stderr("(nil)".to_string()),
        Value::Int(i) => output(&DEFAULT_INPUT_VALUE.to_string(), format!("(integer) {}", i), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::BulkString(s) => output(&DEFAULT_INPUT_VALUE.to_string(), String::from_utf8(s).unwrap(), &DEFAULT_STRING_OUTPUT_FORMAT.to_string()),
        Value::Array(a) => {
            for i in 0..a.len() {
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), a[i].clone(), &String::from(format!("{}) {{stdin}}", i)));
            }
        }
        Value::SimpleString(s) => output(&DEFAULT_INPUT_VALUE.to_string(), s, &DEFAULT_STRING_OUTPUT_FORMAT.to_string()),
        Value::Okay => output(&DEFAULT_INPUT_VALUE.to_string(), "\"OK\"".to_string(), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::Map(m) => {
            for i in 0..m.len() {
                let value = m.get(i).unwrap().clone();
                let format_key = &String::from(format!("{}) 1) {{stdin}}", i));
                let format_value = &String::from(format!(" 2) {{stdin}}"));
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), value.0, &format_key);
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), value.1, &format_value);
            }
        }
        Value::Attribute { data, attributes } => {
            writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), *data, &DEFAULT_OUTPUT_FORMAT.to_string());
            for i in 0..attributes.len() {
                let attribute = attributes.get(i).unwrap().clone();
                let key = attribute.0;
                let value = attribute.1;
                let format_key = &String::from(format!("{}) 1) {{stdin}}", i));
                let format_value = &String::from(format!(" 2) {{stdin}}"));
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), key, &format_key);
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), value, &format_value);
            }
        }
        Value::Set(s) => {
            for i in 0..s.len() {
                let v = s.get(i).unwrap();
                writeln_redis_value_to_stdout(&DEFAULT_INPUT_VALUE.to_string(), v.clone(), &String::from(format!("{}) {{stdin}}", i)));
            }
        }
        Value::Double(d) => output(&DEFAULT_INPUT_VALUE.to_string(), format!("(double) {}", d), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::Boolean(b) => output(&DEFAULT_INPUT_VALUE.to_string(), format!("(bool) {}", b), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::VerbatimString { format: _format, text } => {
            output(&DEFAULT_INPUT_VALUE.to_string(), text, &DEFAULT_STRING_OUTPUT_FORMAT.to_string());
        }
        Value::BigNumber(bn) => output(&DEFAULT_INPUT_VALUE.to_string(), format!("(bigNumber) {}", bn), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::Push { .. } => output(&DEFAULT_INPUT_VALUE.to_string(), "Pushed".to_string(), &DEFAULT_OUTPUT_FORMAT.to_string()),
        Value::ServerError(e) => {
            output(&DEFAULT_INPUT_VALUE.to_string(), format!("server error: {}", e), &DEFAULT_ERROR_OUTPUT_FORMAT.to_string());
        }
        _ => {
            writeln_to_stderr("unsupported redis value type".to_string());
        }
    };
}

pub fn writeln_redis_value_to_stdout(input_value: &String, value: Value, output_format: &String) {
    match value {
        Value::Nil => writeln_to_stdout("Nil".to_string()),
        Value::Int(i) => output(input_value, format!("{}", i), output_format),
        Value::BulkString(s) => output(input_value, String::from_utf8_lossy(s.as_slice()).to_string(), output_format),
        Value::Array(a) => {
            for v in a {
                writeln_redis_value_to_stdout(input_value, v, output_format);
            }
        }
        Value::SimpleString(s) => output(input_value, s, output_format),
        Value::Okay => output(input_value, "OK".to_string(), output_format),
        Value::Map(m) => {
            for (k, v) in m {
                writeln_redis_value_to_stdout(input_value, k, output_format);
                writeln_redis_value_to_stdout(input_value, v, output_format);
            }
        }
        Value::Attribute { data, attributes } => {
            writeln_redis_value_to_stdout(input_value, *data, output_format);
            for (k, v) in attributes {
                writeln_redis_value_to_stdout(input_value, k, output_format);
                writeln_redis_value_to_stdout(input_value, v, output_format);
            }
        }
        Value::Set(s) => {
            for v in s {
                writeln_redis_value_to_stdout(input_value, v, output_format);
            }
        }
        Value::Double(d) => output(input_value, format!("{}", d), output_format),
        Value::Boolean(b) => output(input_value, format!("{}", b), output_format),
        Value::VerbatimString { format: _format, text } => {
            output(input_value, text, output_format);
        }
        Value::BigNumber(bn) => output(input_value, format!("{}", bn), output_format),
        Value::Push { .. } => output(input_value, "Pushed".to_string(), output_format),
        Value::ServerError(e) => {
            output(input_value, format!("server error: {}", e), output_format);
        }
        _ => {
            writeln_to_stderr("unsupported redis value type".to_string());
        }
    };
}

fn output(input_value: &String, output_value: String, output_format: &String) {
    let mut output_str = output_format.clone();
    output_str = output_str.replace("{stdout}", output_value.as_str());
    output_str = output_str.replace("{stdin}", input_value.as_str());
    writeln_to_stdout(output_str);
}

pub fn writeln_to_stdout(str: String) {
    stdout().write(format!("{}\n", str).as_bytes()).unwrap();
}

pub fn writeln_to_stderr(str: String) {
    stderr().write(format!("{}\n", str).as_bytes()).unwrap();
}
