use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::club::Club;
use crate::tour::Rencontres;
use crate::tour::Tour;
use crate::API;

#[derive(Debug, Deserialize, Clone)]
pub struct Poule {
    pub division: String,
    pub numero: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// représente le classement d'une équipe dans la poule
pub struct Classement {
    /// numéro de la poule
    #[serde(rename = "poule")]
    pub numero_poule: u8,
    /// classement dans la poule
    #[serde(rename = "clt")]
    pub classement: u8,
    /// nom de l'équipe
    pub equipe: String,
    /// nombre de matchs joués
    #[serde(rename = "joue")]
    pub joues: u8,
    /// nombre de points
    #[serde(rename = "pts")]
    pub points: u8,
    /// nombre de victoires
    #[serde(rename = "vic")]
    pub victoires: u8,
    /// nombre de défaites
    #[serde(rename = "def")]
    pub defaites: u8,
    /// nombre de nuls
    #[serde(rename = "nul")]
    pub nuls: u8,
    /// nombre de forfaits
    #[serde(rename = "pf")]
    pub forfaits: u8,
    /// nombre de parties gagnées
    #[serde(rename = "pg")]
    pub parties_gagnees: u8,
    /// nombre de parties perdues
    #[serde(rename = "pp")]
    pub parties_perdues: u8,
    /// le numero du club
    #[serde(rename = "numero")]
    pub numero: String,
}

#[derive(Debug, Deserialize)]
/// un ensemble de classements
pub struct Classements {
    #[serde(rename = "classement")]
    /// les classements
    pub classements: Vec<Classement>,
}

impl Poule {
    /// retourne les tours de la poule
    pub async fn get_tours(&self) -> Vec<(u8, Tour)> {
        let request_url = format!(
            "{API}/proxy/xml_result_equ.php?force=1&D1={}&cx_poule={}",
            self.division, self.numero
        );
        log::debug!("Appel API: {request_url}");
        let response = reqwest::get(&request_url)
            .await
            .expect("Impossible de récupérer la poule")
            .text()
            .await
            .unwrap();
        let rencontres: Rencontres = quick_xml::de::from_str(&response).unwrap();
        let mut tours_hashed: HashMap<u8, Tour> = HashMap::new();
        for r in &rencontres.rencontres {
            match tours_hashed.get_mut(&r.tour) {
                Some(tour) => {
                    tour.rencontres.rencontres.push(r.clone());
                }
                None => {
                    tours_hashed.insert(
                        r.tour,
                        Tour {
                            numero: r.tour,
                            date: r.date.clone(),
                            rencontres: Rencontres {
                                rencontres: vec![r.clone()],
                            },
                        },
                    );
                }
            }
        }
        //println!("{:?}", htours);
        let mut tours = Vec::new();
        for i in 1..=u8::try_from(tours_hashed.len()).expect("This number should be a u8") {
            tours.push((i, tours_hashed[&i].clone()));
        }
        tours.sort_by_key(|x| x.0);
        tours
    }

    /// retourne les classements des équipes de la poule
    pub async fn classement(&self) -> Vec<Classement> {
        let request_url = format!(
            "{API}/proxy/xml_result_equ.php?force=1&action=classement&D1={}&cx_poule={}",
            self.division, self.numero
        );
        log::debug!("Appel API: {request_url}");
        let response = reqwest::get(&request_url)
            .await
            .expect("Impossible de récupérer le classement de la poule")
            .text()
            .await
            .unwrap();
        let classement: Classements = quick_xml::de::from_str(&response).unwrap();
        classement.classements
    }

    /// retourne les clubs représentés dans la poule
    pub async fn get_clubs(&self) -> Vec<Club> {
        let classements = self.classement().await;
        let mut clubs: Vec<Club> = vec![];
        for c in classements {
            let club = Club::new(&c.numero).await;
            clubs.push(club);
        }
        clubs
    }
}
