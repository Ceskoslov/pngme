use std::fmt::Display;

use crate::chunk::Chunk;
pub struct Png {
    pub signature: [u8; 8],
    pub chunks: Vec<Chunk>,
}

impl Png {
    const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Self {
            signature: Self::STANDARD_HEADER,
            chunks,
        }
    }

    fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk, String> {
        if let Some(pos) = self.chunks.iter().position(|c| c.chunk_type().to_string() == chunk_type) {
            Ok(self.chunks.remove(pos))
        } else {
            Err(format!("Chunk type '{}' not found", chunk_type))
        }
    }

    fn header(&self) -> [u8; 8] {
        self.signature
    }

    fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.chunk_type().to_string() == chunk_type)
    }

    fn as_bytes(&self) -> Vec<u8> {
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
        let signature = bytes[0..8].try_into().unwrap();
        if signature != Self::STANDARD_HEADER {
            return Err("Invalid PNG signature".to_string());
        }
        // Placeholder for chunk parsing logic
        Ok(Self {
            signature,
            chunks: Vec::new(),
        })
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PNG with {} chunks", self.chunks.len())
    }
}