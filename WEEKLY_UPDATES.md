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
   
---
  
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

---

### Réunion du 17/02/22

Discussion pour faire le livrable 1. 
Écriture de la présentation du projet avec les objectifs (livrable minimal et idéal), diagramme de séquence et rétroplanning.  
La fonction callsign() qui découpe et traduit le message de 56 bits 
Avancement dans l'apprentissage du RUST

#### Objectifs pour la semaine prochaine : 
* Terminer le livrable 1 (pour le 18/02)
* Avancer dans l'apprentissage du RUST
* Se répartir des fonctions simple à coder afin d'avancer dans le projet et de s'habituer à coder en RUST
* Réfléchir à des solutions pour l'interface graphique  

---

### Réunion du 24/02/22  

* Le premier livrable a été téléversé (18/02) sur moodle
* Code des fonctions permettant d'extraire la position (latitude, longitude) du message
* Discussion pour l'interface graphique : nous allons commencer à regarder imgui
  
#### Objectifs pour la semaine prochaine :  
* Coder la fonction vitesse
* Découverte du framework imgui pour l'interface graphique
* S'intéresser à la réception des signaux avec rust ( recherche d'un framework adapté ? )
* (Continuer à apprendre le RUST)

--- 

*Dans ce document, nous décrirons le travail effectué chaque semaine ainsi que les objectifs pour la semaine d'après.*
