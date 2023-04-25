use serde::Serialize;

use crate::typo_issue::TypoIssue;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Blocker,
    Critical,
    Major,
    Minor,
    Info,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Location {
    pub path: String,
    pub positions: Position,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Position {
    pub begin: PositionInfo,
    //pub end: PositionInfo,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PositionInfo {
    pub line: u32,
    pub column: u32,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CodeClimateIssue {
    pub severity: Severity,
    pub description: String,
    pub fingerprint: String,
    pub location: Location,
}

impl From<TypoIssue> for CodeClimateIssue {
    fn from(source: TypoIssue) -> Self {
        let severity = match source.is_filename {
            true => Severity::Major,
            false => Severity::Info,
        };

        let description = format!(
            "Found a typo: '{}'. Did you mean '{}'?",
            source.typo, source.fix
        );
        let fingerprint = format!("{:x}", md5::compute(source.raw));
        let path = source.file;
        let line = source.line;
        let column = source.column;

        CodeClimateIssue {
            severity,
            description,
            fingerprint,
            location: Location {
                path,
                positions: Position {
                    begin: PositionInfo { line, column },
                },
            },
        }
    }
}
