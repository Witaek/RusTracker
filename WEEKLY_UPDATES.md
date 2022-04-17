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

### Réunion du 03/03/22 (point rencontre)  

* Préférable de scinder le projet en deux sous-projets : 'backend' et 'frontend'
* Discussion autout de l'interface graphique : on part sur imgui avec l'utilisation de ZeroMQ pour séparer le 'backend' et le 'frontend'
* Réception des messages : voir le framword rtlsdr_mt (pas actualisé) sinon se tourner vers soapysdr
* Choix d'utiliser des array de booléens pour représenter les messages binaires plutôt que des string pour gain de mémoire
* Discussion autour du livrable 2
* Prochain point rencontre le 16/03 à 16h sur le campus

#### Objectifs pour la semaine prochaine (ou prochain point rencontre) :  

* Adapter les fonctions avec le systèmes de tableaux de booléens
* Intégration de tests rust pour les fonctions définies
* Préparation du livrable 2 (premier prototype) : 
  * Backend : test des fonctions sur plusieurs messages binaires, réception des messages, démodulation (traitement I&Qs)
  * Frontend : Afficher une carte avec des points mouvants, possibilité d'afficher les infos sur les points (vitesse, position, etc)

---

### Réunion du 16/03/22 (point rencontre)  

* Discussion autour des bases de données : se tourner vers SQLLite3 plutôt 
* Clarifier le code aux endroits où il devient un peu lourd et peu intuitif (cf tracking.rs)
* Regarder des librairies de logging pour éventuellement générer des logs
* Possibilité d'utiliser la docu rust pour notre livrable final
* Problème réglé pour imgui :
  * sudo apt install libxcb-render*
  * sudo apt install libxcb-xfixes*
  * sudo apt install libxcb-shape*
* Discussion autour de l'interface graphique, il faut choisir entre :  
  * imgui qui fonctionnerait avec ZeroMQ : il faut chercher comment générer une carte (openstreetmap), les tiles
  * interface web avec HTML/CSS, Javascript : utilisation de leaflet pour générer facilement une carte, utiliser des websockets pour communiquer entre back/front

#### Objectifs pour la semaine prochaine (libvrable 2) :  

* Se décider sur l'interface graphique
* Finaliser (ou presque) le livrable 2, à savoir :  
  * Backend : réception, démodulation, et interprétation des messages ADS-B --> FAIT
  * Frontend : générer une carte, avec à minima des points dynamiques et la possibilité d'afficher leur vitesse, position etc...
* Clarifier la partie réception du code
* Voir les histoires de position avec un seul message

---

### Réunion du 29/03/2022 (rapide) :  

* Choix d'utiliser Yew et leaflet-rs pour l'interface graphique
* Prise en main de Yew et leaflet-rs, quelques difficultées rencontrées (à voir au point rencontre du 31/03)
* ZeroMQ : grâce à l'exemple fournit par  Rémy Grünblatt un code minimal d'emission/reception serveur/client a été mis au point et fonctionne correctement (pour l'instant en local)
* Point rencontre encadrant pour régler quelques problèmes liés à l'interface graphique : 31/03/2022 à 8h


#### Objectifs pour la semaine prochaine :

* Réussir à afficher une icone d'avion et à gérer sa position sur la carte
* Gérer proprement la réception des messages côté client
* Gérer l'affichage des données côté client

---

### Réunion du 14/04/2022 (point rencontre) :  

* Discuission autour de l'interface graphique, notamment de Yew qui pose pas mal de problèmes : on laisse tomber le rust pour le web et on part sur du web pur / javascript.
* Discussion autour de l'organisation du projet : il faut que chacun s'organise pour participer au projet (frontend, backend, connexion à internet, rapport, slides soutenance etc...).
* Discussion autour de la forme du rapport : possibilité d'utiliser des fichiers markdown puis de les convertir avec pandoc (solution retenue = forme d'un rust book https://rust-lang.github.io/mdBook/index.html ).
* La distribution des rôles :  
--> Guilhem : termine le backend côté web  
--> Romain : s'occupe du côté serveur, mise en place d'une vm pour pouvoir faire passer les sockets par internet et utiliser plusieurs antennes  
--> Artur : s'occupe du frontend, apparence de l'interface web  
--> Bastian : rédige le rapport sous la forme de rustbook  

#### Obejctifs pour le prochain point :  
* Absolument avoir une interface graphique minimale : points qui bougent, carte, etc...
* Gérer la connexion internet du serveur
* Gérer le côté backend de l'interface graphique (interprétation des messages de position, vitesse, etc afin de positionner les avions sur la carte)

### Concrètement on en est où ? 
* Backend : on reçoit et démodule les signaux ADS-B captés par nos antennes sur Rustracker (branche RecpServ projet source). Après avoir testé leur intégrité (parité et CRC) les messages sont ensuite envoyés sous forme de binaires sur le serveur (branche RecpServ projet rustracker) avec zeromq qui les interprète (pour obtenir, position, vitesse, etc) et retranscrit les informations dans un fichier geojson qui permet à leaflet de fonctionner en temps réel.
* Serveur : nous disposons désormais d'une vm opérationnelle sur les serveurs MiNET (ip : 157.159.195.63). Les tests de transmissions/réception de socket ou de messages via tcp sont concluants.
* Frontent : en cours de création, c'est le plus gros chantier restant. https://github.com/GuilhemHnr/trackui

--- 

*Dans ce document, nous décrirons le travail effectué chaque semaine ainsi que les objectifs pour la semaine d'après.*
