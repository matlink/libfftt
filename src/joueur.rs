use serde::Deserialize;

#[derive(Debug)]
pub enum JoueurError {
    NotFound,
    AucunePartie,
}

#[derive(Debug, Deserialize, Clone)]
// Représente un joueur
pub struct Joueur {
    // numéro de licence
    #[serde(rename = "licence")]
    pub _licence: String,
    pub nom: String,
    pub prenom: String,
    // ses points de début de saison
    #[serde(rename = "initm")]
    pub points_init: f32,
    // ses points en début de phase
    pub point: f32,
    // ses points actuels
    #[serde(rename = "virtual")]
    pub r#_virtual: f32,
}

const API: &str = "http://fftt.dafunker.com/v1";

impl Joueur {
    // créé le joueur en récupérant les données depuis SPID
    // licence représente le numéro de licence du joueur
    pub async fn new(licence: &str) -> Result<Joueur, JoueurError> {
        let mut j = Self::api_joueur(licence).await.map_err(|e| {
            log::error!("Erreur de récupération du joueur N°{licence} : {e}");
            JoueurError::NotFound
        })?;

        log::debug!("Joueur N°{licence}: {}", j.display());
        Ok(j)
    }

    pub fn display(&self) -> String {
        format!("{} {}", self.prenom, self.nom)
    }

    // récupère un joueur via l'API
    async fn api_joueur(licence: &str) -> Result<Joueur, reqwest::Error> {
        let request_url = format!("{API}/joueur/{licence}");
        let response = reqwest::get(&request_url).await?;
        let response = response.text().await?;
        let joueur =
            serde_json::from_str(&response).expect("Erreur lors de la désérialisation du joueur");
        Ok(joueur)
    }
}
