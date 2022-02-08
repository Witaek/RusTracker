# WEEKLY UPDATES

### Réunion du 27/01/22

Création du repo Git (licence GNU GPL v3)  
Discussion sur nos objectifs concernant le projet, livrable minimal, livrable amélioré (cf README)  
Mise à jour du README
Langage : RUST (formation en cours) plus compliqué à prendre en main que Java/Python mais plus optimisé pour notre projet.

**Discussion sur la structure :**  
  
--> Classe radar :  
* Qui hérite de thread  
* S'éxécute à l'infini
* Reçoit des essages en permanence et créée un objet avion dès qu'il est détecté
* Extrait le message en cherchant le préambule classique de l'ADS-B

--> Classe Avion :  
  
* Message en binaire
* Data : 56 bits, soit la vitesse, soit le numéro de vol, soit les coordonnées 
* Traduit les données 
* Crée une carte toutes les X secondes

Dans ce document, nous décrirons le travail effectué chaque semaine ainsi que les objectifs pour la semaine d'après.
