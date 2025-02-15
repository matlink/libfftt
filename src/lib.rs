mod club;
mod joueur;
const API: &str = "http://fftt.dafunker.com/v1";

#[cfg(test)]
mod tests {
    use crate::joueur::Joueur;

    #[tokio::test]
    async fn test_joueur() {
        let joueur = Joueur::new("3421810")
            .await
            .expect("Erreur lors de la récupération du joueur");
        assert_eq!(joueur.nom, "LEBRUN");
        assert_eq!(joueur.prenom, "Felix");
        assert_eq!(joueur._licence, "3421810");
    }
}
