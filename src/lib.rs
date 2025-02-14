mod joueur;

#[cfg(test)]
mod tests {
    use crate::joueur::Joueur;

    use super::*;

    #[tokio::test]
    async fn test_joueur() {
        let joueur = Joueur::new("3421810").await.expect("Erreur lors de la récupération du joueur");
        assert_eq!(joueur.nom, "LEBRUN");
        assert_eq!(joueur.prenom, "Felix");
        assert_eq!(joueur._licence, "3421810");
    }
}
