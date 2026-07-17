// tests/fuse_mod.rs

use super::*;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use fuse::{Filesystem, Operations};
use crypto::{Encrypt, Decrypt};
use file_system::FileSystem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuse_mod() {
        let mut fuse = FileSystem::new();
        let file_system = fuse.get_file_system();
        let fuse_operations = file_system.get_fuse_operations();

        // Test création de fichier
        test_create_file(&fuse_operations);

        // Test création de répertoire
        test_create_directory(&fuse_operations);

        // Test lecture de fichier
        test_read_file(&fuse_operations);

        // Test écriture de fichier
        test_write_file(&fuse_operations);

        // Test suppression de fichier
        test_remove_file(&fuse_operations);

        // Test suppression de répertoire
        test_remove_directory(&fuse_operations);
    }

    fn test_create_file(operations: &Operations) {
        let file_name = "test_file";
        let file_path = PathBuf::from(file_name);
        let file = operations.open(&file_path, 0).unwrap();
        let mut file_stream = file.stream();
        file_stream.write(b"Hello, world!").unwrap();
        let mut file_content = String::new();
        file_stream.read_to_string(&mut file_content).unwrap();
        assert_eq!(file_content, "Hello, world!");
    }

    fn test_create_directory(operations: &Operations) {
        let dir_name = "test_directory";
        let dir_path = PathBuf::from(dir_name);
        operations.mkdir(&dir_path, 0).unwrap();
        operations.readdir(&dir_path).unwrap();
    }

    fn test_read_file(operations: &Operations) {
        let file_name = "test_file";
        let file_path = PathBuf::from(file_name);
        let file = operations.open(&file_path, 0).unwrap();
        let mut file_stream = file.stream();
        let mut file_content = String::new();
        file_stream.read_to_string(&mut file_content).unwrap();
        assert_eq!(file_content, "Hello, world!");
    }

    fn test_write_file(operations: &Operations) {
        let file_name = "test_file";
        let file_path = PathBuf::from(file_name);
        let mut file = operations.open(&file_path, 0).unwrap();
        let mut file_stream = file.stream();
        file_stream.write(b"Bonjour, monde!").unwrap();
        let mut file_content = String::new();
        file_stream.read_to_string(&mut file_content).unwrap();
        assert_eq!(file_content, "Bonjour, monde!");
    }

    fn test_remove_file(operations: &Operations) {
        let file_name = "test_file";
        let file_path = PathBuf::from(file_name);
        operations.remove(&file_path).unwrap();
    }

    fn test_remove_directory(operations: &Operations) {
        let dir_name = "test_directory";
        let dir_path = PathBuf::from(dir_name);
        operations.rmdir(&dir_path).unwrap();
    }
}