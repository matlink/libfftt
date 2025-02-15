pub mod club;
pub mod joueur;
pub const API: &str = "http://fftt.dafunker.com/v1";

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
}
