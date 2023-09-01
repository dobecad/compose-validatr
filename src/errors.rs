#[derive(Debug)]
pub enum ValidationError {
    MissingField(String),
    InvalidValue(String),
    InvalidCompose(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "Missing field: {}", field),
            ValidationError::InvalidValue(value) => write!(f, "Invalid value: {}", value),
            ValidationError::InvalidCompose(value) => write!(f, "Invalid compose file: {}", value),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        ValidationErrors { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn all_errors(&self) -> &[ValidationError] {
        &self.errors
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}
