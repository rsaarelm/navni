use anyhow::bail;
use base64::{Engine, engine::general_purpose};

pub struct Directory(String);

impl Directory {
    pub fn config(application_name: &str) -> anyhow::Result<Self> {
        Ok(Directory(format!("{application_name}/config/")))
    }

    pub fn data(application_name: &str) -> anyhow::Result<Self> {
        Ok(Directory(format!("{application_name}/data/")))
    }

    pub fn list(&self) -> Vec<String> {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();

        let mut ret = Vec::new();
        for i in 0..storage.len() {
            let Some(key) = storage.key(i) else { continue };
            if let Some(key) = key.strip_prefix(&self.0) {
                ret.push(key.into());
            }
        }
        ret
    }

    pub fn exists(&self, name: &str) -> bool {
        self.read(name).is_ok()
    }

    pub fn read(&self, name: &str) -> anyhow::Result<String> {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();

        if let Some(s) = storage.get(&format!("{}{name}", self.0)) {
            Ok(s)
        } else {
            bail!("not found")
        }
    }

    pub fn write(&mut self, name: &str, text: &str) -> anyhow::Result<()> {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();

        storage.set(&format!("{}{name}", self.0), text);
        Ok(())
    }

    pub fn read_bytes(&self, name: &str) -> anyhow::Result<Vec<u8>> {
        let base64 = self.read(name)?;
        let data = general_purpose::STANDARD_NO_PAD.decode(&base64)?;
        Ok(data)
    }

    pub fn write_bytes(
        &mut self,
        name: &str,
        data: &[u8],
    ) -> anyhow::Result<()> {
        let enc = general_purpose::STANDARD_NO_PAD.encode(data);
        self.write(name, &enc)
    }

    pub fn delete(&mut self, name: &str) -> anyhow::Result<()> {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();

        storage.remove(&format!("{}{name}", self.0));
        Ok(())
    }
}
