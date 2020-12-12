use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;
use pyo3::Python;

use sqlparser::dialect::*;
use sqlparser::parser::Parser;

fn string_to_dialect(dialect: &str) -> Box<dyn Dialect> {
    match dialect {
        "generic" => Box::new(GenericDialect {}),
        "ansi" => Box::new(AnsiDialect {}),
        "ms" => Box::new(MsSqlDialect {}),
        "postgres" => Box::new(PostgreSqlDialect {}),
        _ => Box::new(GenericDialect {})
    }
}



#[pyfunction]
fn parse_sql(_py: Python, dialect: &str, sql: &str) -> PyResult<String> {
    let chosen_dialect = string_to_dialect(dialect);
    let parse_result =
        Parser::parse_sql(&*chosen_dialect, sql);

    let json_output = match parse_result {
        Ok(statements) => serde_json::to_string(&statements).unwrap_or("[]".to_string()),
        Err(_e) => return Err(PyValueError::new_err("Parsing failed."))
    };

    Ok(json_output)
}

#[pymodule]
fn sqloxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_sql, m)?)?;
    Ok(())
}
