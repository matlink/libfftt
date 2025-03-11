use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::date::Date;

/// La rencontre d'une équipe contre une autre
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rencontre {
    #[serde(rename = "libelle", deserialize_with = "deserialize_numero_tour")]
    /// le tour de la rencontre
    pub tour: u8,
    #[serde(rename = "equa")]
    /// l'équipe A (receveur)
    pub a: String,
    #[serde(rename = "equb")]
    /// l'équipe B (extérieur)
    pub b: String,
    #[serde(default)]
    /// le score de l'équipe A
    pub scorea: String,
    #[serde(default)]
    /// le score de l'équipe B
    pub scoreb: String,
    #[serde(rename = "dateprevue")]
    /// la date de la rencontre
    pub date: Date,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// un ensemble de rencontres
pub struct Rencontres {
    #[serde(rename = "tour")]
    pub rencontres: Vec<Rencontre>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Tour {
    pub numero: u8,
    pub date: Date,
    pub rencontres: Rencontres,
}

fn deserialize_numero_tour<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let numero = s
        .split('°')
        .last()
        .expect("Numéro de tour manquant")
        .split(' ')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    Ok(numero)
}
