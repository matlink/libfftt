use std::mem;

use serde::Deserialize;

use crate::API;

#[derive(Debug, Deserialize, Clone)]
// un club de tennis de table
pub struct Club {
    // le numéro du club
    pub numero: String,
    pub nom: String,
    pub nomsalle: String,
    pub adressesalle1: String,
    pub adressesalle2: String,
    pub adressesalle3: String,
    pub codepsalle: String,
    pub villesalle: String,
    pub latitude: String,
    pub longitude: String,
}

impl Club {
    // créé le club en interrogeant l'API avec l'id donné
    pub async fn new(idclub: &str) -> Club {
        let request_url = format!("{API}/proxy/xml_club_detail.php?club={idclub}");
        let response = reqwest::get(&request_url)
            .await
            .expect("Impossible de récupérer les informations du club")
            .text()
            .await
            .expect("Impossible de récupérer les informations du club");
        let mut doc: DocumentClub =
            quick_xml::de::from_str(&response).expect("Erreur lors de la désérialisation du club");
        let club = &mut doc.club[0];
        log::debug!("Club trouvé : {}", club.nom);
        // les clubs en France ont une longitude < latitude, on inverse si erreur de saisie par le club
        if club.latitude < club.longitude {
            mem::swap(&mut club.latitude, &mut club.longitude);
        }
        Club {
            nom: club.nom.clone(),
            numero: club.numero.clone(),
            adressesalle1: club.adressesalle1.clone(),
            adressesalle2: club.adressesalle2.clone(),
            adressesalle3: club.adressesalle3.clone(),
            codepsalle: club.codepsalle.clone(),
            latitude: club.latitude.clone(),
            longitude: club.longitude.clone(),
            nomsalle: club.nomsalle.clone(),
            villesalle: club.villesalle.clone(),
        }
    }
}


#[derive(Debug, Deserialize)]
// représente le document XML retourné par l'API
struct DocumentClub {
    // l'ensemble des licences de tous les joueurs
    club: Vec<Club>,
}