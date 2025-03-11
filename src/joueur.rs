use std::collections::HashMap;

use serde::Deserialize;

use crate::{partie::Partie, API};

#[derive(Debug)]
pub enum JoueurError {
    NotFound,
    AucunePartie,
}

#[derive(Debug, Deserialize, Clone)]
/// Représente un joueur
pub struct Joueur {
    /// numéro de licence
    pub licence: String,
    /// nom du joueur
    pub nom: String,
    /// prénom du joueur
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

impl Joueur {
    /// créé le joueur en récupérant les données depuis SPID
    /// licence représente le numéro de licence du joueur
    pub async fn new(licence: &str) -> Result<Joueur, JoueurError> {
        let j = Self::api_joueur(licence).await.map_err(|e| {
            log::error!("Erreur de récupération du joueur N°{licence} : {e}");
            JoueurError::NotFound
        })?;

        log::debug!("Joueur N°{licence}: {}", j.display());
        Ok(j)
    }

    pub fn display(&self) -> String {
        format!("{} {}", self.prenom, self.nom)
    }

    /// récupère un joueur via l'API à partir de son numéro de licence
    async fn api_joueur(licence: &str) -> Result<Joueur, reqwest::Error> {
        let request_url = format!("{API}/joueur/{licence}");
        let response = reqwest::get(&request_url).await?;
        let response = response.text().await?;
        let joueur =
            serde_json::from_str(&response).expect("Erreur lors de la désérialisation du joueur");
        Ok(joueur)
    }

    /// récupère les parties du joueur
    pub async fn api_parties(&self) -> Option<Partie> {
        let request_url = format!("{API}/parties/{}", self.licence);
        let response = match reqwest::get(&request_url).await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Impossible de récupérer les parties : {e}");
                return None;
            }
        };
        let response = match response.text().await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Impossible de récupérer les parties : {e}");
                return None;
            }
        };
        let parties: HashMap<String, Vec<Partie>> =
            serde_json::from_str(&response).expect("Erreur lors de la désérialisation des parties");
        let parties = Partie::sum_parties(&parties["list"]);
        parties.regrouper_journees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_api_joueur() {
        // Simule une réponse valide.
        let joueur_data = r#"{
            "licence": "12345",
            "nom": "Doe",
            "prenom": "John",
            "initm": 1000.0,
            "point": 1100.0,
            "virtual": 1200.0,
            "parties": {"processed": 0, "num_matchs": 0, "ex": 0, "journees": []},
            "progression": {}
        }"#;

        // Remplacez `reqwest::get` par un mock dans les tests réels.
        let joueur: Joueur = serde_json::from_str(joueur_data).unwrap();
        assert_eq!(joueur.nom, "Doe");
        assert_eq!(joueur.prenom, "John");
        assert_eq!(joueur.points_init, 1000.0);
        assert_eq!(joueur.point, 1100.0);
        assert_eq!(joueur.r#_virtual, 1200.0);
    }

    #[tokio::test]
    async fn test_api_parties() {
        // Simule une réponse pour les parties.
        let partie_data = r#"{
            "list": [
                { "processed": 1, "num_matchs": 2, "ex": 5.0,"journees": [
                    {
                        "epreuve": "FED_Championnat de France par Equipes Masculin",
                        "date": "29/11/2024",
                        "ex": 2.0,
                        "matchs": [
                            {
                                "nom": "DOE John",
                                "coeff": 1.0,
                                "vdf": 0,
                                "ex": 4.0,
                                "licence": "123456",
                                "p": "8"
                            },
                            {
                                "nom": "DHUILE Jean",
                                "coeff": 1.0,
                                "vdf": 1,
                                "ex": -5.0,
                                "licence": "654321",
                                "p": "10"
                            }
                        ]
                    }
                ] },
                { "processed": 0, "num_matchs": 0, "ex": 3.0, "journees": [] }
            ]
        }"#;

        let parties: HashMap<String, Vec<Partie>> = serde_json::from_str(partie_data).unwrap();
        assert!(parties.contains_key("list"));
        assert_eq!(parties["list"].len(), 2);
    }

    #[tokio::test]
    async fn test_new() {
        // Test de la création d'un nouveau joueur avec des données simulées.
        let licence = "12345";

        let result = Joueur::new(licence).await;
        assert!(result.is_ok());
        let joueur = result.unwrap();
        assert_eq!(joueur.licence, licence);
    }

    #[test]
    fn test_display() {
        let joueur = Joueur {
            licence: "12345".to_string(),
            nom: "Doe".to_string(),
            prenom: "John".to_string(),
            points_init: 1000.0,
            point: 1100.0,
            r#_virtual: 1200.0,
        };
        assert_eq!(joueur.display(), "John Doe");
    }
}
