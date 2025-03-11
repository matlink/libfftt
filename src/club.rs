use std::mem;

use serde::Deserialize;

use crate::{equipe::Equipe, joueur::Joueur, API};

#[derive(Debug, Deserialize, Clone)]
/// un club de tennis de table
pub struct Club {
    /// le numéro identifiant le club. Commence généralement par le numéro de département
    pub numero: String,
    /// Le nom du club
    pub nom: String,
    /// Le nom de la salle accueillant les compétitions
    pub nomsalle: String,
    /// L'adresse de la 1ère salle de compétition
    pub adressesalle1: String,
    /// L'adresse de la 2ème salle de compétition
    pub adressesalle2: String,
    /// L'adresse de la 3ème salle de compétition
    pub adressesalle3: String,
    /// Le code postal de la salle
    pub codepsalle: String,
    /// La ville de la salle
    pub villesalle: String,
    /// La latitude de la position GPS de la salle
    pub latitude: String,
    /// La longitude de la position GPS de la salle
    pub longitude: String,
}

impl Club {
    /// Créé le club en interrogeant l'API avec l'id donné
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

    /// Retourne l'ensemble des joueurs du club en interrogeant l'API
    pub async fn api_joueurs(&self) -> Vec<Joueur> {
        log::info!("Récupération des joueurs du club...");
        let request_url = format!("{API}/proxy/xml_licence_b.php?club={}", self.numero);
        let response = reqwest::get(&request_url)
            .await
            .expect("Impossible de récupérer la liste des joueurs du club")
            .text()
            .await
            .expect("Impossible de récupérer la liste des joueurs du club");
        // la réponse est en xml
        log::info!("Traitement de la réponse...");
        let doc: Document = quick_xml::de::from_str(&response)
            .expect("Erreur lors de la désérialisation des joueurs");

        // on initialise tous les joueurs
        log::info!("Initialisation des joueurs...");
        let mut joueurs = vec![];
        for x in &doc.licence {
            if let Ok(j) = Joueur::new(&x.licence).await {
                joueurs.push(j);
            }
        }
        joueurs
    }

    /// Retourne les équipes du club engagées en compétition en interrogeant l'API
    pub async fn api_equipes(&self) -> Vec<Equipe> {
        log::info!("Récupération des équipes du club...");
        let request_url = format!("{API}/club/{}/equipes", self.numero);
        let response = reqwest::get(&request_url)
            .await
            .expect("Impossible de récupérer la liste des équipes du club")
            .text()
            .await
            .expect("Impossible de récupérer la liste des équipes du club");
        // la réponse est en json
        log::info!("Traitement de la réponse...");
        serde_json::from_str(&response).expect("Erreur lors du parsing des équipes du club")
    }
}

#[derive(Debug, Deserialize)]
// représente le document XML retourné par l'API
struct DocumentClub {
    // l'ensemble des licences de tous les joueurs
    club: Vec<Club>,
}

#[derive(Debug, Deserialize)]
// représente le document XML retourné par l'API
struct Document {
    // l'ensemble des licences de tous les joueurs
    licence: Vec<Licence>,
}

#[derive(Debug, Deserialize)]
// un joueur
struct Licence {
    // le numéro de licence
    licence: String,
    // son prénom
    // prenom: String,
}
