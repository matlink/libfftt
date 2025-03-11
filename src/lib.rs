//! Une bibliothèque Rust pour interagir avec l'API de la Fédération Française de Tennis de Table (FFTT), permettant d'accéder aux informations sur les joueurs, clubs, compétitions et classements.
//!
//! # Exemples
//! ## Récupérer un club
//! ```
//! use libfftt::club::Club;
//!
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let montpellier = Club::new("11340010").await;
//! println!("Nom du club : {}", montpellier.nom);
//! # })
//! ```
//! Affichera :
//! ```text
//! Nom du club : MONTPELLIER TT
//! ```
//!
//! ## Récupérer un joueur
//! ```
//! use libfftt::joueur::Joueur;
//!
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let felix = Joueur::new("3421810").await.expect("Erreur lors de la récupération du joueur");
//! println!("Nom du joueur : {}", felix.prenom);
//! # })
//! ```
//! Affichera :
//! ```text
//! Nom du joueur : Felix
//! ```
//!
//! ## Récupérer les joueurs d'un club
//! ```
//! use libfftt::club::Club;
//!
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let montpellier = Club::new("11340010").await;
//! let joueurs = montpellier.api_joueurs().await;
//! # })
//! ```
//! On peut ensuite itérer dessus :
//! ```
//! # use libfftt::club::Club;
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! #   let montpellier = Club::new("11340010").await;
//! #   let joueurs = montpellier.api_joueurs().await;
//! joueurs.iter().map(|j| println!("Joueur : {} {}, points : {}", j.prenom, j.nom, j.point));
//! # })
//! ```
//!
//! ## Récupérer les équipes d'un club
//! ```
//! use libfftt::club::Club;
//!
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let montpellier = Club::new("11340010").await;
//! let equipes = montpellier.api_equipes().await;
//! #   equipes.iter().map(|e| println!("Équipe : {}", e.nom));
//! # })
//! ```
//! On peut ensuite itérer dessus :
//! ```
//! # use libfftt::club::Club;
//! # use tokio;
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! # let montpellier = Club::new("11340010").await;
//! # let equipes = montpellier.api_equipes().await;
//! equipes.iter().map(|e| println!("Équipe : {}", e.nom));
//! # })
//! ```
//!
/// Club
pub mod club;
/// Objet date permettant les comparaisons
pub mod date;
/// Équipe en compétition
pub mod equipe;
/// Joueur
pub mod joueur;
/// Journée de compétition
pub mod journee;
/// Ensemble des rencontres d'une saison
pub mod partie;
/// Poule de l'équipe en compétition
pub mod poule;
/// Tour d'une phase
pub mod tour;
/// Base des endpoints d'API
pub const API: &str = "http://fftt.dafunker.com/v1";
/// Mois séparant la phase 1 de la phase 2
pub const MISAISON_MOIS: u8 = 1;
/// Jour du mois séparant la phase 1 de la phase 2
pub const MISAISON_JOUR: u8 = 11;
/// Mois du début de saison
pub const DEBSAISON_MOIS: u8 = 9;
/// Jour du début de saison
pub const DEBSAISON_JOUR: u8 = 1;

#[cfg(test)]
mod tests {
    use crate::{club::Club, joueur::Joueur};

    #[tokio::test]
    async fn test_joueur() {
        let joueur = Joueur::new("3421810")
            .await
            .expect("Erreur lors de la récupération du joueur");
        assert_eq!(joueur.nom, "LEBRUN");
        assert_eq!(joueur.prenom, "Felix");
        assert_eq!(joueur.licence, "3421810");
    }

    #[tokio::test]
    async fn test_club() {
        let club = Club::new("11340010").await;
        assert_eq!(club.nom, "MONTPELLIER TT");
    }

    #[tokio::test]
    async fn test_club_joueurs() {
        let club = Club::new("11340010").await;
        assert_eq!(club.nom, "MONTPELLIER TT");
        let joueurs = club.api_joueurs().await;
        assert!(joueurs
            .iter()
            .any(|j| j.nom == "LEBRUN" && j.prenom == "Felix"));
    }

    #[tokio::test]
    async fn test_club_equipes() {
        let club = Club::new("11340010").await;
        assert_eq!(club.nom, "MONTPELLIER TT");
        let equipes = club.api_equipes().await;
        assert!(equipes
            .iter()
            .any(|j| j.nom.starts_with("MONTPELLIER TT 1")));
    }
}
