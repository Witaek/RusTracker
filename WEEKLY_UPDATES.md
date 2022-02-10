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
  
  
### Réunion du 10/02/22

Discussion autour du projet, notamment autour de la structure du projet ainsi que des grandes étapes.  
Discussion autour du rétroplanning que nous allons présenter dans le premier livrable. 

Début du code de fonctions simples pour prendre en main RUST : 
* Une fonction qui découpe un message binaire de 56 bits afin de récupérer les sections de bits correspondant aux informations sur le vol
* Une fonction qui gère la base de données contenant les informations relatives aux avions (modèle, moteur, etc)

#### Objectifs pour la semaine prochaine :  
* Terminer le code des fonctions précédemment citées
* Mettre au point un rétroplanning aboutit ainsi que la présentation du projet pour le livrable 1
* Continuer à se former sur RUST (suivit du book rust + mooc )
* Prise en main de l'outil git pour ceux qui n'y sont pas encore familiers  

*Dans ce document, nous décrirons le travail effectué chaque semaine ainsi que les objectifs pour la semaine d'après.*
