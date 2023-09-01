use std::{fs, path::PathBuf};

use anyhow::Context;

pub struct Directory(PathBuf);

impl Drop for Directory {
    /// Delete the directory if it was left empty.
    fn drop(&mut self) {
        if self.0.exists() {
            let _ = (|| -> Result<_, std::io::Error> {
                if fs::read_dir(&self.0)?.next().is_none() {
                    fs::remove_dir(&self.0)?;
                }
                Ok(())
            })();
        }
    }
}

impl Directory {
    pub fn config(application_name: &str) -> anyhow::Result<Self> {
        let xdg_data_dir = dirs::config_dir()
            .with_context(|| "Failed to get default config dir")?;

        // Create the directory if it doesn't exist yet.
        let path = xdg_data_dir.join(application_name);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        Ok(Directory(path))
    }

    pub fn data(application_name: &str) -> anyhow::Result<Self> {
        let xdg_data_dir = dirs::data_dir()
            .with_context(|| "Failed to get default data dir")?;

        // Create the directory if it doesn't exist yet.
        let path = xdg_data_dir.join(application_name);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        Ok(Directory(path))
    }

    pub fn list(&self) -> Vec<String> {
        let Ok(entries) = fs::read_dir(&self.0) else {
            return Default::default();
        };

        entries
            .filter_map(Result::ok)
            .filter(|e| matches!(e.file_type(), Ok(t) if t.is_file()))
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_owned()))
            .collect()
    }

    pub fn read(&self, name: &str) -> anyhow::Result<String> {
        let ret = fs::read_to_string(self.0.join(name))?;
        Ok(ret)
    }

    pub fn write(&mut self, name: &str, text: &str) -> anyhow::Result<()> {
        fs::write(self.0.join(name), text)?;
        Ok(())
    }

    pub fn read_bytes(&self, name: &str) -> anyhow::Result<Vec<u8>> {
        let ret = fs::read(self.0.join(name))?;
        Ok(ret)
    }

    pub fn write_bytes(
        &mut self,
        name: &str,
        data: &[u8],
    ) -> anyhow::Result<()> {
        fs::write(self.0.join(name), data)?;
        Ok(())
    }

    pub fn delete(&mut self, name: &str) -> anyhow::Result<()> {
        fs::remove_file(self.0.join(name))?;
        Ok(())
    }
}
