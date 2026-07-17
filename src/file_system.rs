//! Module de gestion du système de fichiers, incluant la gestion de la mémoire cache

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use fuse::{FileAttr, FileStatus, Filesystem, FilesystemInfo, Request, Reply};
use kernel_api::{Context, Error};
use crate::crypto::{decrypt, encrypt};
use crate::fuse_mod::{FuseMod, FuseModError};

/// Structure représentant la mémoire cache du système de fichiers
pub struct Cache {
    /// Dictionnaire de fichiers en cache
    pub files: HashMap<PathBuf, FileAttr>,
    /// Dictionnaire de répertoires en cache
    pub dirs: HashMap<PathBuf, FileAttr>,
}

impl Cache {
    /// Crée une nouvelle instance de la mémoire cache
    pub fn new() -> Self {
        Cache {
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    /// Vérifie si un fichier est en cache
    pub fn is_file_in_cache(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// Vérifie si un répertoire est en cache
    pub fn is_dir_in_cache(&self, path: &Path) -> bool {
        self.dirs.contains_key(path)
    }

    /// Ajoute un fichier à la mémoire cache
    pub fn add_file_to_cache(&mut self, path: PathBuf, attr: FileAttr) {
        self.files.insert(path, attr);
    }

    /// Ajoute un répertoire à la mémoire cache
    pub fn add_dir_to_cache(&mut self, path: PathBuf, attr: FileAttr) {
        self.dirs.insert(path, attr);
    }

    /// Supprime un fichier de la mémoire cache
    pub fn remove_file_from_cache(&mut self, path: &Path) {
        self.files.remove(path);
    }

    /// Supprime un répertoire de la mémoire cache
    pub fn remove_dir_from_cache(&mut self, path: &Path) {
        self.dirs.remove(path);
    }
}

/// Structure représentant le système de fichiers
pub struct FileSystem {
    /// Instance de la mémoire cache
    cache: Arc<Mutex<Cache>>,
    /// Instance du module de gestion de FUSE
    fuse_mod: Arc<dyn FuseMod>,
    /// Chemin racine du système de fichiers
    root_path: PathBuf,
}

impl FileSystem {
    /// Crée une nouvelle instance du système de fichiers
    pub fn new(root_path: PathBuf, fuse_mod: Arc<dyn FuseMod>) -> Self {
        FileSystem {
            cache: Arc::new(Mutex::new(Cache::new())),
            fuse_mod,
            root_path,
        }
    }

    /// Gère la lecture d'un fichier
    fn read_file(&self, path: &Path, reply: Reply) -> Result<(), Error> {
        if self.cache.is_file_in_cache(path) {
            let attr = self.cache.lock().unwrap().files.get(path).unwrap();
            reply.data(attr.size as usize);
            reply.attr(attr);
            return Ok(());
        }
        let file = fs::File::open(path)?;
        let attr = FileAttr {
            ino: 1,
            size: file.metadata()?.len(),
            blocks: 1,
            atime: file.metadata()?.modified().unwrap(),
            mtime: file.metadata()?.modified().unwrap(),
            ctime: file.metadata()?.created().unwrap(),
            uid: 0,
            gid: 0,
            mode: 0o444,
        };
        let encrypted_data = encrypt(file.metadata()?.len(), file.metadata()?.contents().unwrap())?;
        self.cache.lock().unwrap().add_file_to_cache(path.to_path_buf(), attr);
        reply.data(encrypted_data);
        reply.attr(attr);
        Ok(())
    }

    /// Gère l'écriture d'un fichier
    fn write_file(&self, path: &Path, data: &[u8], reply: Reply) -> Result<(), Error> {
        let file = fs::File::create(path)?;
        let attr = FileAttr {
            ino: 1,
            size: data.len() as u64,
            blocks: 1,
            atime: file.metadata()?.modified().unwrap(),
            mtime: file.metadata()?.modified().unwrap(),
            ctime: file.metadata()?.created().unwrap(),
            uid: 0,
            gid: 0,
            mode: 0o444,
        };
        let decrypted_data = decrypt(data.len(), data)?;
        file.write_all(&decrypted_data)?;
        self.cache.lock().unwrap().add_file_to_cache(path.to_path_buf(), attr);
        reply.attr(attr);
        Ok(())
    }

    /// Gère la suppression d'un fichier
    fn remove_file(&self, path: &Path) -> Result<(), Error> {
        fs::remove_file(path)?;
        self.cache.lock().unwrap().remove_file_from_cache(path);
        Ok(())
    }
}

impl Filesystem for FileSystem {
    fn getattr(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        reply.attr(FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
            atime: std::time::SystemTime::now(),
            mtime: std::time::SystemTime::now(),
            ctime: std::time::SystemTime::now(),
            uid: 0,
            gid: 0,
            mode: 0o444,
        });
        Ok(())
    }

    fn readdir(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        let dir = fs::read_dir(self.root_path.clone())?;
        let entries: Vec<FileAttr> = dir
            .map(|entry| {
                let entry = entry?;
                let attr = entry.file_name().to_str().unwrap().to_string();
                let path = self.root_path.join(attr);
                if entry.file_type().unwrap().is_dir() {
                    FileAttr {
                        ino: 1,
                        size: 0,
                        blocks: 0,
                        atime: std::time::SystemTime::now(),
                        mtime: std::time::SystemTime::now(),
                        ctime: std::time::SystemTime::now(),
                        uid: 0,
                        gid: 0,
                        mode: 0o555,
                    }
                } else {
                    FileAttr {
                        ino: 1,
                        size: 0,
                        blocks: 0,
                        atime: std::time::SystemTime::now(),
                        mtime: std::time::SystemTime::now(),
                        ctime: std::time::SystemTime::now(),
                        uid: 0,
                        gid: 0,
                        mode: 0o444,
                    }
                }
            })
            .flatten()
            .collect();
        reply.entry(entries);
        Ok(())
    }

    fn open(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.root_path.clone())?;
        let attr = FileAttr {
            ino: 1,
            size: file.metadata()?.len(),
            blocks: 1,
            atime: file.metadata()?.modified().unwrap(),
            mtime: file.metadata()?.modified().unwrap(),
            ctime: file.metadata()?.created().unwrap(),
            uid: 0,
            gid: 0,
            mode: 0o444,
        };
        reply.fd(file.as_raw_fd());
        reply.attr(attr);
        Ok(())
    }

    fn read(&self, _req: &Request, _in: &Path, _out: &Path, reply: Reply) -> Result<(), Error> {
        self.read_file(_in, reply)
    }

    fn write(&self, _req: &Request, _in: &Path, _out: &Path, data: &[u8], reply: Reply) -> Result<(), Error> {
        self.write_file(_in, data, reply)
    }

    fn truncate(&self, _req: &Request, _in: &Path, _size: u64, reply: Reply) -> Result<(), Error> {
        reply.truncate(_size);
        Ok(())
    }

    fn unlink(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        self.remove_file(_in);
        reply.removed(true);
        Ok(())
    }

    fn release(&self, _req: &Request, _in: &Path, _out: &Path, _flags: u32, reply: Reply) -> Result<(), Error> {
        reply.release(true);
        Ok(())
    }

    fn release_file(&self, _req: &Request, _in: &Path, _out: &Path, _flags: u32, reply: Reply) -> Result<(), Error> {
        reply.release_file(true);
        Ok(())
    }

    fn getattr_all(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        let dir = fs::read_dir(self.root_path.clone())?;
        let entries: Vec<FileAttr> = dir
            .map(|entry| {
                let entry = entry?;
                let attr = entry.file_name().to_str().unwrap().to_string();
                let path = self.root_path.join(attr);
                if entry.file_type().unwrap().is_dir() {
                    FileAttr {
                        ino: 1,
                        size: 0,
                        blocks: 0,
                        atime: std::time::SystemTime::now(),
                        mtime: std::time::SystemTime::now(),
                        ctime: std::time::SystemTime::now(),
                        uid: 0,
                        gid: 0,
                        mode: 0o555,
                    }
                } else {
                    FileAttr {
                        ino: 1,
                        size: 0,
                        blocks: 0,
                        atime: std::time::SystemTime::now(),
                        mtime: std::time::SystemTime::now(),
                        ctime: std::time::SystemTime::now(),
                        uid: 0,
                        gid: 0,
                        mode: 0o444,
                    }
                }
            })
            .flatten()
            .collect();
        reply.entry(entries);
        Ok(())
    }

    fn statfs(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        reply.statfs(FileStatus {
            bsize: 512,
            frsize: 1024,
            blocks: 1,
            bfree: 1,
            bavail: 1,
            files: 1,
           _ffree: 1,
            favail: 1,
            fsid: 1,
            namemax: 255,
        });
        Ok(())
    }

    fn destroy(&self, _req: &Request, _in: &Path, reply: Reply) -> Result<(), Error> {
        reply.destroy(true);
        Ok(())
    }
}