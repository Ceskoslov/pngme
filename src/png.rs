use std::fmt::Display;

pub use crate::chunk::Chunk;

pub struct Png {
    pub signature: [u8; 8],
    pub chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Self {
            signature: Self::STANDARD_HEADER,
            chunks,
        }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk, String> {
        if let Some(pos) = self
            .chunks
            .iter()
            .position(|c| c.chunk_type().to_string() == chunk_type)
        {
            Ok(self.chunks.remove(pos))
        } else {
            Err(format!("Chunk type '{}' not found", chunk_type))
        }
    }

    pub fn header(&self) -> [u8; 8] {
        self.signature
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|c| c.chunk_type().to_string() == chunk_type)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.signature);
        for chunk in &self.chunks {
            bytes.extend(chunk.as_bytes());
        }
        bytes
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err("Not enough bytes to form a PNG".to_string());
        }
        let signature: [u8; 8] = bytes[0..8].try_into().unwrap();
        if signature != Self::STANDARD_HEADER {
            return Err("Invalid PNG signature".to_string());
        }

        let mut offset = 8;
        let mut chunks = Vec::new();

        while offset < bytes.len() {
            if offset + 4 > bytes.len() {
                return Err("Truncated chunk length".to_string());
            }
            let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
            let chunk_end = offset + 12 + length;
            if chunk_end > bytes.len() {
                return Err(format!(
                    "Chunk extends beyond file: need {} bytes, have {}",
                    chunk_end,
                    bytes.len()
                ));
            }
            let chunk = Chunk::try_from(&bytes[offset..chunk_end])?;
            chunks.push(chunk);
            offset = chunk_end;
        }

        Ok(Self { signature, chunks })
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PNG with {} chunks", self.chunks.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();
        chunks.push(chunk_from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(chunk_from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(chunk_from_strings("LASt", "I am the last chunk").unwrap());
        chunks
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, String> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();
        Ok(Chunk::new(chunk_type, data))
    }

    #[test]
    fn test_from_chunks() {
        let png = testing_png();
        assert_eq!(png.chunks().len(), 3);
    }

    #[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|c| c.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        assert!(png.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|c| c.as_bytes())
            .collect();

        let bytes: Vec<u8> = [13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        assert!(png.is_err());
    }

    #[test]
    fn test_invalid_chunk() {
        let mut chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|c| c.as_bytes())
            .collect();

        // Corrupt the CRC of the first chunk
        let idx = chunk_bytes.len() - 4;
        chunk_bytes[idx] ^= 0xFF;

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        assert!(png.is_err());
    }

    #[test]
    fn test_list_chunks() {
        let png = testing_png();
        let chunks = png.chunks();
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn test_chunk_by_type() {
        let png = testing_png();
        let chunk = png.chunk_by_type("FrSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "FrSt");
        assert_eq!(&chunk.data_as_string().unwrap(), "I am the first chunk");
    }

    #[test]
    fn test_append_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_strings("TeSt", "Message").unwrap());
        let chunk = png.chunk_by_type("TeSt").unwrap();
        assert_eq!(&chunk.data_as_string().unwrap(), "Message");
    }

    #[test]
    fn test_remove_first_chunk() {
        let mut png = testing_png();
        let chunk = png.remove_first_chunk("FrSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "FrSt");
        assert!(png.chunk_by_type("FrSt").is_none());
    }

    #[test]
    fn test_as_bytes() {
        let png = testing_png();
        let bytes = png.as_bytes();
        let png2 = Png::try_from(bytes.as_ref()).unwrap();
        assert_eq!(png.chunks().len(), png2.chunks().len());
    }

    #[test]
    fn test_png_trait_impls() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|c| c.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png: Png = TryFrom::try_from(bytes.as_ref()).unwrap();
        let _png_string = format!("{}", png);
    }
}
