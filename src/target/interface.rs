use anyhow::Context;
use std::io::Write;
use std::path::PathBuf;

pub trait TargetImpl: Sized {
    const NAME: &'static str;
    const EXTENSION: &'static str;
    fn to_string_pretty(&self) -> anyhow::Result<String>;
}

pub trait Target
where
    Self: 'static,
{
    fn output_path(&self, name: &str) -> PathBuf;
    fn write_file(&self, name: &str) -> anyhow::Result<()>;
}

impl<T> Target for T
where
    T: TargetImpl + 'static,
{
    fn output_path(&self, name: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(Self::NAME);
        path.set_file_name(name);
        path.set_extension(Self::EXTENSION);
        path
    }
    fn write_file(&self, name: &str) -> anyhow::Result<()> {
        let path = self.output_path(name);
        let content = self.to_string_pretty()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())
            .context("Failed to write file.")?;
        Ok(())
    }
}
