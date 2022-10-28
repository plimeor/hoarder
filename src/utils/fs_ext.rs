use std::fs;
use std::io;
use std::os;
use std::path::PathBuf;

pub fn copy(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    ensure_dir(dest)?;

    if src.is_file() {
        fs::copy(src, dest)?;
        return Ok(());
    } else if src.is_dir() {
        src.read_dir()?
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .for_each(|item| {
                let mut dest = dest.clone();
                dest.push(item.file_name());
                copy(&item.path(), &dest).unwrap();
            });
    }
    Ok(())
}

pub fn ensure_dir(target: &PathBuf) -> io::Result<()> {
    if target.exists() || target.parent().is_none() {
        return Ok(());
    }

    let parent_dir = target.parent().unwrap();
    if parent_dir.exists() {
        Ok(())
    } else {
        fs::create_dir_all(parent_dir)
    }
}

pub fn remove(target: &PathBuf) -> io::Result<()> {
    if target.is_file() {
        fs::remove_file(target)
    } else if target.is_dir() {
        fs::remove_dir_all(target)
    } else {
        Ok(())
    }
}

pub fn symlink(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    os::unix::fs::symlink(src, dest)
}
