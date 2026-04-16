use std::io;
use std::fs;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

#[cfg(not(any(
    feature = "toml",
)))]
compile_error!("you must enable at least one of the file type features");

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read file")]
    ReadError(#[source] io::Error),
    #[error("Failed to write to file")]
    WriteError(#[source] io::Error),
    #[error("Failed to create parent directory")]
    DirectoryCreationError(#[source] io::Error),
    #[cfg(feature = "toml")]
    #[error("Failed to deserialize TOML")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[cfg(feature = "toml")]
    #[error("Failed to serialize TOML")]
    TomlSerializeError(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FileType {
    #[cfg(feature = "toml")]
    Toml,
}

impl FileType {
    pub fn filename<P: AsRef<OsStr>>(&self, base: P) -> PathBuf {
        let mut filename = PathBuf::from(base.as_ref());
        filename.set_extension(self.extension());
        filename
    }

    pub fn extension(&self) -> &str {
        match self {
            #[cfg(feature = "toml")]
            FileType::Toml => "toml",
        }
    }
}

pub fn load<T: Default + DeserializeOwned + Serialize>(file: &Path, ext: FileType) -> Result<T> {
    let resource: T = match fs::read(&file) {
        Ok(data) => match ext {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::from_slice(&data)?,
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            let resource = T::default();
            store(file, ext, &resource)?;
            resource
        },
        Err(e) => return Err(Error::ReadError(e)),
    };
    Ok(resource)
}

pub fn store<T: Serialize>(file: &Path, ext: FileType, resource: &T) -> Result<()> {
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(Error::DirectoryCreationError)?;
    }
    let data: String = match ext {
        #[cfg(feature = "toml")]
        FileType::Toml => toml::to_string(resource)?,
    };
    fs::write(file, data).map_err(Error::WriteError)
}

pub trait Resolver {
    fn resolve<P: AsRef<OsStr>>(&self, base: P, ext: FileType) -> PathBuf;
}

#[cfg(feature = "etcetera")]
impl<S: etcetera::AppStrategy> Resolver for S {
    fn resolve<P: AsRef<OsStr>>(&self, base: P, ext: FileType) -> PathBuf {
        self.in_config_dir(ext.filename(base))
    }
}

pub trait Repository {
    fn load<T: Default + DeserializeOwned + Serialize>(&self, base: &str, ext: FileType) -> Result<T>;
    fn store<T: Serialize>(&self, base: &str, ext: FileType, resource: &T) -> Result<()>;
}

impl<R: Resolver> Repository for R {
    fn load<T: Default + DeserializeOwned + Serialize>(&self, base: &str, ext: FileType) -> Result<T> {
        load(&self.resolve(base, ext), ext)
    }

    fn store<T: Serialize>(&self, base: &str, ext: FileType, resource: &T) -> Result<()> {
        store(&self.resolve(base, ext), ext, resource)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::FileType;

    #[test]
    fn test_extensions() {
        #[cfg(feature = "toml")]
        assert_eq!(FileType::Toml.filename("config"), Path::new("config.toml"));
    }
}
