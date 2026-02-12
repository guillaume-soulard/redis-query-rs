use redis::Value;
use std::io::{stderr, stdout, Write};

pub fn writeln_redis_value_to_stdout(input_value: &String, value: Value, output_format: &String)  {
    match value {
        Value::Nil => writeln_to_stderr("Nil".to_string()),
        Value::Int(i) => output(input_value, format!("{}", i), output_format),
        Value::BulkString(s) => output(input_value, String::from_utf8(s).unwrap(), output_format),
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
