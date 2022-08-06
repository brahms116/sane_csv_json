pub type Result<T, E = Whoops> = core::result::Result<T, E>;

pub struct WhoopsBuilder {
    err_type: Option<String>,
    context: Option<String>,
    why: Option<String>,
    suggestion: Option<String>,
}

impl WhoopsBuilder {
    pub fn new() -> Self {
        Self {
            err_type: None,
            context: None,
            why: None,
            suggestion: None,
        }
    }

    pub fn err_type(mut self, err_type: &str) -> Self {
        self.err_type = Some(err_type.into());
        self
    }

    pub fn context(mut self, context: &str) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn why(mut self, why: &str) -> Self {
        self.why = Some(why.into());
        self
    }

    pub fn suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    pub fn build(self) -> Whoops {
        Whoops {
            err_type: self.err_type.unwrap_or("unknown error type".into()),
            context: self.context.unwrap_or("unknown context".into()),
            why: self.why.unwrap_or("unknown reason".into()),
            suggestion: self
                .suggestion
                .unwrap_or("no suggestions, contact author for help".into()),
        }
    }
}

#[derive(Debug)]
pub struct Whoops {
    pub err_type: String,
    pub context: String,
    pub why: String,
    pub suggestion: String,
}

impl Whoops {
    pub fn augment_context(&mut self, context: &str) -> &mut Self {
        self.context = format!("{}, {}", context, self.context);
        self
    }
}

impl std::fmt::Display for Whoops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
--- ERROR ---

Error Type: {}

Context: {}

Why: {}

Suggestion: {}

-------------

",
            self.err_type, self.context, self.why, self.suggestion
        )
    }
}

impl std::error::Error for Whoops {}
