# FUSE-Encrypted-Filesystem
[![Rust](https://img.shields.io/badge/Rust-1.64.0-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/username/FUSE-Encrypted-Filesystem/actions/workflows/ci.yml/badge.svg)](https://github.com/username/FUSE-Encrypted-Filesystem/actions/workflows/ci.yml)

## Description

FUSE-Encrypted-Filesystem est un système de fichiers chiffré basé sur FUSE, implémenté en Rust. Il permet de créer un système de fichiers chiffré qui peut être utilisé pour stocker des données sensibles de manière sécurisée.

## Fonctionnalités

* Chiffrement des données à l'aide de l'algorithme AES
* Déchiffrement des données à l'aide de la clé de déchiffrement
* Création de fichiers et de répertoires chiffrés
* Gestion de la mémoire cache pour améliorer les performances
* Support de la gestion de FUSE pour intégrer le système de fichiers chiffré dans le système d'exploitation

## Installation

Pour installer FUSE-Encrypted-Filesystem, vous devez avoir Rust installé sur votre ordinateur. Vous pouvez l'installer en suivant les instructions disponibles sur le site officiel de Rust.

Une fois Rust installé, vous pouvez cloner le repository Git de FUSE-Encrypted-Filesystem en utilisant la commande suivante :
```bash
git clone https://github.com/username/FUSE-Encrypted-Filesystem.git
```
Ensuite, vous pouvez installer le projet en exécutant la commande suivante dans le répertoire du projet :
```bash
cargo build
```
## Usage

Pour utiliser FUSE-Encrypted-Filesystem, vous devez créer un répertoire chiffré en exécutant la commande suivante :
```bash
fuse-encrypted --create /chemin/vers/reseau
```
Cela créera un répertoire chiffré à l'emplacement spécifié.

Vous pouvez ensuite copier des fichiers dans le répertoire chiffré en utilisant la commande suivante :
```bash
cp fichier.txt /chemin/vers/reseau
```
Les fichiers seront chiffrés automatiquement lors de la copie.

Pour déchiffrer les fichiers, vous pouvez utiliser la commande suivante :
```bash
fuse-encrypted --decrypt /chemin/vers/reseau
```
Cela déchiffrera les fichiers dans le répertoire chiffré.

## Architecture du projet

Le projet FUSE-Encrypted-Filesystem est structuré en plusieurs modules :

* `src/main.rs` : Fichier principal, contenant la logique d'implémentation de FUSE
* `src/fuse_mod.rs` : Module de gestion de FUSE, incluant la création de fichiers et de répertoires
* `src/crypto.rs` : Module de cryptographie, implémentant les algorithmes de chiffrement et de déchiffrement
* `src/file_system.rs` : Module de gestion du système de fichiers, incluant la gestion de la mémoire cache

## Contribuer

Si vous souhaitez contribuer au projet FUSE-Encrypted-Filesystem, vous pouvez créer une branche de développement en exécutant la commande suivante :
```bash
git checkout -b nom-de-la-branche
```
Vous pouvez ensuite modifier le code en utilisant votre éditeur de code préféré. Une fois que vous avez terminé les modifications, vous pouvez commit les changements en exécutant la commande suivante :
```bash
git add .
git commit -m "Message de commit"
```
Enfin, vous pouvez push les changements sur le repository distant en exécutant la commande suivante :
```bash
git push origin nom-de-la-branche
```
## Licence

FUSE-Encrypted-Filesystem est licencié sous la licence MIT. Vous pouvez télécharger la licence en cliquant [ici](https://opensource.org/licenses/MIT).