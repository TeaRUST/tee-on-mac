use serde::{Deserialize, Serialize}; 
use wasmer_wasi::{
  
  state::{self, WasiFile, WasiFsError},
  types,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingWrapper {
    pub wasm_module_name: String,
}

// std io trait boiler plate so we can implement WasiFile
// LoggingWrapper is a write-only type so we just want to immediately
// fail when reading or Seeking
impl std::io::Read for LoggingWrapper {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_to_string(&mut self, _buf: &mut String) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
    fn read_exact(&mut self, _buf: &mut [u8]) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not read from logging wrapper",
        ))
    }
}
impl std::io::Seek for LoggingWrapper {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "can not seek logging wrapper",
        ))
    }
}
impl std::io::Write for LoggingWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write_all(buf)
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(b"[")?;
        out.write(self.wasm_module_name.as_bytes())?;
        out.write(b"]: ")?;
        out.write_fmt(fmt)
    }
}

// the WasiFile methods aren't relevant for a write-only Stdout-like implementation
// we must use typetag and serde so that our trait objects can be safely Serialized and Deserialized
#[typetag::serde]
impl WasiFile for LoggingWrapper {
    fn last_accessed(&self) -> u64 {
        0
    }
    fn last_modified(&self) -> u64 {
        0
    }
    fn created_time(&self) -> u64 {
        0
    }
    fn size(&self) -> u64 {
        0
    }
    fn set_len(&mut self, _len: u64) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn unlink(&mut self) -> Result<(), WasiFsError> {
        Ok(())
    }
    fn bytes_available(&self) -> Result<usize, WasiFsError> {
        // return an arbitrary amount
        Ok(1024)
    }
}