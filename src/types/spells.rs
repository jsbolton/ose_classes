use std::fmt::Display;

use serde::Deserialize;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct Spells {
    #[serde(rename = "1")]
    pub first: Option<i64>,

    #[serde(rename = "2")]
    pub second: Option<i64>,

    #[serde(rename = "3")]
    pub third: Option<i64>,

    #[serde(rename = "4")]
    pub fourth: Option<i64>,

    #[serde(rename = "5")]
    pub fifth: Option<i64>,

    #[serde(rename = "6")]
    pub sixth: Option<i64>,
}

impl Display for Spells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut spells = Vec::<String>::default();

        if let Some(f) = self.first {
            spells.push(format!("1:{}", f))
        }
        if let Some(f) = self.second {
            spells.push(format!("2:{}", f))
        }
        if let Some(f) = self.third {
            spells.push(format!("3:{}", f))
        }
        if let Some(f) = self.fourth {
            spells.push(format!("4:{}", f))
        }
        if let Some(f) = self.fifth {
            spells.push(format!("5:{}", f))
        }
        if let Some(f) = self.sixth {
            spells.push(format!("6:{}", f))
        }

        write!(f, "{}", spells.join(" | "))?;

        Ok(())
    }
}
