//! Module de gestion de FUSE, incluant la création de fichiers et de répertoires

use std::fs;
use std::path::PathBuf;
use fuse_rust::{Filesystem, Request, Reply, ReplyAttr};
use crate::crypto::{encrypt_data, decrypt_data};
use crate::file_system::{FilesystemState, create_file, create_directory};

/// Fonction de création de fichiers FUSE
///
/// Cette fonction crée un fichier de type FUSE et défini ses attributs.
///
/// # Arguments
///
/// * `path`: Chemin vers le fichier à créer
/// * `mode`: Mode de création du fichier (lecture, écriture, exécution, etc.)
/// * `uid`: ID de l'utilisateur propriétaire du fichier
/// * `gid`: ID du groupe propriétaire du fichier
/// * `size`: Taille du fichier (en octets)
/// * `atime`: Temps de modification du fichier
/// * `mtime`: Temps de modification du fichier
/// * `ctime`: Temps de création du fichier
/// * `blocks`: Nombre de blocs du fichier (en séquences de 512 octets)
///
/// # Retour
///
/// * `Reply` si la création du fichier a réussi
/// * `Err` si la création du fichier a échoué
pub fn createFuseFile(path: &PathBuf, mode: u32, uid: u32, gid: u32, size: u64, atime: i64, mtime: i64, ctime: i64, blocks: u64) -> Result<Reply, Err> {
    let fs = FilesystemState::get_instance();
    let file = create_file(path, mode, uid, gid, size, atime, mtime, ctime, blocks)?;
    let encrypted_data = encrypt_data(file.data())?;
    fs.set_file(path, encrypted_data)?;
    Ok(ReplyAttr {
        ino: 1,
        mode,
        nlink: 1,
        uid,
        gid,
        rdev: 0,
        size,
        atime: fs.get_atime(path)?,
        mtime: fs.get_mtime(path)?,
        ctime: fs.get_ctime(path)?,
        blocks: 1,
        nblocks: blocks,
        flags: 0,
        gen: 0,
        file_type: 0,
    })
}

/// Fonction de création de répertoires FUSE
///
/// Cette fonction crée un répertoire de type FUSE et défini ses attributs.
///
/// # Arguments
///
/// * `path`: Chemin vers le répertoire à créer
/// * `mode`: Mode de création du répertoire (lecture, écriture, exécution, etc.)
/// * `uid`: ID de l'utilisateur propriétaire du répertoire
/// * `gid`: ID du groupe propriétaire du répertoire
/// * `size`: Taille du répertoire (en octets)
/// * `atime`: Temps de modification du répertoire
/// * `mtime`: Temps de modification du répertoire
/// * `ctime`: Temps de création du répertoire
///
/// # Retour
///
/// * `Reply` si la création du répertoire a réussi
/// * `Err` si la création du répertoire a échoué
pub fn createFuseDirectory(path: &PathBuf, mode: u32, uid: u32, gid: u32, size: u64, atime: i64, mtime: i64, ctime: i64) -> Result<Reply, Err> {
    let fs = FilesystemState::get_instance();
    let directory = create_directory(path, mode, uid, gid, size, atime, mtime, ctime)?;
    fs.set_directory(path, directory)?;
    Ok(ReplyAttr {
        ino: 1,
        mode,
        nlink: 2,
        uid,
        gid,
        rdev: 0,
        size,
        atime: fs.get_atime(path)?,
        mtime: fs.get_mtime(path)?,
        ctime: fs.get_ctime(path)?,
        blocks: 1,
        nblocks: 1,
        flags: 0,
        gen: 0,
        file_type: 0,
    })
}

/// Fonction de fermeture du système de fichiers FUSE
///
/// Cette fonction ferme le système de fichiers FUSE et libère les ressources associées.
///
/// # Arguments
///
/// * `path`: Chemin vers le système de fichiers à fermer
///
/// # Retour
///
/// * `Reply` si la fermeture du système de fichiers a réussi
/// * `Err` si la fermeture du système de fichiers a échoué
pub fn closeFuseFilesystem(path: &PathBuf) -> Result<Reply, Err> {
    let fs = FilesystemState::get_instance();
    fs.close_filesystem(path)?;
    Ok(Reply {
        error: 0,
        file: None,
    })
}