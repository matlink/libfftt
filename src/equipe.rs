use serde::{Deserialize, Deserializer};

use crate::poule::Poule;

#[derive(Debug, Deserialize, Clone)]
/// une équipe engagée en compétition
pub struct Equipe {
    /// numéro de l'équipe
    #[serde(rename = "idequipe")]
    pub id: String,

    /// nom de l'équipe
    #[serde(rename = "libequipe")]
    pub nom: String,

    /// libellé de l'épreuve
    #[serde(rename = "libepr")]
    pub _epreuve: String,

    /// nom de la division
    #[serde(rename = "libdivision")]
    pub _division: String,

    /// poule de l'équipe
    #[serde(rename = "liendivision", deserialize_with = "deserialize_poule")]
    pub poule: Poule,
}

fn deserialize_poule<'de, D>(deserializer: D) -> Result<Poule, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s = s.to_string();
    let fields: Vec<&str> = s.split('&').collect();
    let division = fields
        .iter()
        .filter(|x| x.starts_with("D1"))
        .last()
        .unwrap()
        .split('=')
        .last()
        .unwrap();
    let cx_poule = fields
        .iter()
        .filter(|x| x.starts_with("cx_poule"))
        .last()
        .unwrap()
        .split('=')
        .last()
        .unwrap();
    Ok(Poule {
        division: division.to_string(),
        numero: cx_poule.to_string(),
    })
}
