use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypoIssue {
    pub is_filename: bool,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub typo: String,
    pub fix: String,
    pub raw: String,
}

#[derive(Debug)]
pub enum ConvertError {
    InvalidFormatError,
    //RegexError,
}

impl TryFrom<String> for TypoIssue {
    type Error = ConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(.+\.[^:]+):(\d+)(?::(\d+))?:\s`(.+)`\s->\s`(.+)`").unwrap();

        if !re.is_match(value.as_str()) {
            return Err(ConvertError::InvalidFormatError);
        }
        
        let groups = re.captures(value.as_str()).unwrap();

        let file = groups.get(1).unwrap().as_str().to_owned();
        let line = groups.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let column = match groups.get(3) {
            Some(matching) => matching.as_str().parse::<u32>().unwrap(),
            None => 0
        };
        let typo = groups.get(4).unwrap().as_str().to_owned();
        let fix = groups.get(5).unwrap().as_str().to_owned();

        Ok(TypoIssue {
            is_filename: false,
            file,
            line,
            column,
            typo,
            fix,
            raw: value,
        })
    }
}
