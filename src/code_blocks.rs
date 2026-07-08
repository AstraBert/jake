use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum ScriptLanguage {
    Javascript,
    Typescript,
    Python,
    Ruby,
    Php,
    Lua,
    Perl,
}

#[derive(Debug)]
pub struct LanguageNotSupportedError {
    message: String,
}

impl std::error::Error for LanguageNotSupportedError {}

impl Display for LanguageNotSupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl FromStr for ScriptLanguage {
    type Err = LanguageNotSupportedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "py" | "python" => Ok(Self::Python),
            "ts" | "typescript" => Ok(Self::Typescript),
            "js" | "javascript" => Ok(Self::Javascript),
            "rb" | "ruby" => Ok(Self::Ruby),
            "lua" => Ok(Self::Lua),
            "perl" => Ok(Self::Perl),
            "php" => Ok(Self::Php),
            _ => Err(LanguageNotSupportedError {
                message: format!("{} is not a supported language for jake scripts", s),
            }),
        }
    }
}

impl ScriptLanguage {
    pub fn command(&self, script: &str) -> anyhow::Result<String> {
        let quoted_script = shlex::try_quote(script)?;
        match self {
            Self::Javascript => Ok(format!("node -e {}", quoted_script)),
            Self::Lua => Ok(format!("lua -e {}", quoted_script)),
            Self::Python => Ok(format!("python3 -c {}", quoted_script)),
            Self::Perl => Ok(format!("perl -e {}", quoted_script)),
            Self::Php => Ok(format!("php -r {}", quoted_script)),
            Self::Ruby => Ok(format!("ruby -e {}", quoted_script)),
            Self::Typescript => Ok(format!("npx tsx -e {}", quoted_script)),
        }
    }
}
