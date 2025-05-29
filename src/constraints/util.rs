use crate::evaluator::{eval_error::EvalError, object::Object};

pub fn extract_int(object: &Object) -> Result<i32, EvalError> {
    match object {
        Object::Int(value) => Ok((*value) as i32),
        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
    }
}

pub fn extract_float(object: &Object) -> Result<f32, EvalError> {
    match object {
        Object::Float(value) => Ok(*value),
        obj => Err(EvalError::type_error(
            "float".to_owned(),
            format!("{}", obj),
        )),
    }
}

pub fn extract_range(object: &Object) -> Result<(isize, isize), EvalError> {
    match object {
        Object::Range(start, end) => Ok((*start, *end)),
        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
    }
}
