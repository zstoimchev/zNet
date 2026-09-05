use crate::crypto::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use std::io;

pub(crate) const PROTOCOL_VERSION: u16 = 1;
pub(crate) const NONCE_LENGTH: usize = 32;

pub(crate) type Nonce = [u8; NONCE_LENGTH];
pub(crate) type SignatureBytes = [u8; SIGNATURE_LENGTH];

const HELLO_KIND: u8 = 1;
const ACK_KIND: u8 = 2;
const FINISH_KIND: u8 = 3;

#[derive(Debug, Clone)]
pub(crate) enum Handshake {
    Hello {
        version: u16,
        listen_port: u16,
        public_key: [u8; PUBLIC_KEY_LENGTH],
        nonce: Nonce,
    },

    Ack {
        version: u16,
        listen_port: u16,
        public_key: [u8; PUBLIC_KEY_LENGTH],
        nonce: Nonce,
        signature: SignatureBytes,
    },

    Finish {
        signature: SignatureBytes,
    },
}

impl Handshake {
    pub(crate) fn hello(
        listen_port: u16,
        public_key: [u8; PUBLIC_KEY_LENGTH],
        nonce: Nonce,
    ) -> Self {
        Self::Hello {
            version: PROTOCOL_VERSION,
            listen_port,
            public_key,
            nonce,
        }
    }

    pub(crate) fn ack(
        listen_port: u16,
        public_key: [u8; PUBLIC_KEY_LENGTH],
        nonce: Nonce,
        signature: SignatureBytes,
    ) -> Self {
        Self::Ack {
            version: PROTOCOL_VERSION,
            listen_port,
            public_key,
            nonce,
            signature,
        }
    }

    pub(crate) fn finish(signature: SignatureBytes) -> Self {
        Self::Finish { signature }
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        match self {
            Self::Hello {
                version,
                listen_port,
                public_key,
                nonce,
            } => {
                let mut payload = Vec::with_capacity(70);

                payload.push(HELLO_KIND);
                payload.extend_from_slice(&version.to_be_bytes());
                payload.extend_from_slice(&listen_port.to_be_bytes());
                payload.extend_from_slice(public_key);
                payload.extend_from_slice(nonce);

                payload
            }

            Self::Ack {
                version,
                listen_port,
                public_key,
                nonce,
                signature,
            } => {
                let mut payload = Vec::with_capacity(134);

                payload.push(ACK_KIND);
                payload.extend_from_slice(&version.to_be_bytes());
                payload.extend_from_slice(&listen_port.to_be_bytes());
                payload.extend_from_slice(public_key);
                payload.extend_from_slice(nonce);
                payload.extend_from_slice(signature);

                payload
            }

            Self::Finish { signature } => {
                let mut payload = Vec::with_capacity(65);

                payload.push(FINISH_KIND);
                payload.extend_from_slice(signature);

                payload
            }
        }
    }

    pub(crate) fn decode(payload: &[u8]) -> io::Result<Self> {
        let kind = payload
            .first()
            .copied()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty handshake payload"))?;

        match kind {
            HELLO_KIND => Self::decode_hello(payload),
            ACK_KIND => Self::decode_ack(payload),
            FINISH_KIND => Self::decode_finish(payload),

            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown handshake type",
            )),
        }
    }

    fn decode_hello(payload: &[u8]) -> io::Result<Self> {
        if payload.len() != 70 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Hello length",
            ));
        }

        let version = u16::from_be_bytes([payload[1], payload[2]]);

        let listen_port = u16::from_be_bytes([payload[3], payload[4]]);

        let mut public_key = [0u8; PUBLIC_KEY_LENGTH];
        public_key.copy_from_slice(&payload[5..38]);

        let mut nonce = [0u8; NONCE_LENGTH];
        nonce.copy_from_slice(&payload[38..70]);

        Ok(Self::Hello {
            version,
            listen_port,
            public_key,
            nonce,
        })
    }

    fn decode_ack(payload: &[u8]) -> io::Result<Self> {
        if payload.len() != 134 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Ack length",
            ));
        }

        let version = u16::from_be_bytes([payload[1], payload[2]]);

        let listen_port = u16::from_be_bytes([payload[3], payload[4]]);

        let mut public_key = [0u8; PUBLIC_KEY_LENGTH];
        public_key.copy_from_slice(&payload[5..38]);

        let mut nonce = [0u8; NONCE_LENGTH];
        nonce.copy_from_slice(&payload[38..70]);

        let mut signature = [0u8; SIGNATURE_LENGTH];
        signature.copy_from_slice(&payload[70..134]);

        Ok(Self::Ack {
            version,
            listen_port,
            public_key,
            nonce,
            signature,
        })
    }

    fn decode_finish(payload: &[u8]) -> io::Result<Self> {
        if payload.len() != 65 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Finish length",
            ));
        }

        let mut signature = [0u8; SIGNATURE_LENGTH];
        signature.copy_from_slice(&payload[1..65]);

        Ok(Self::Finish { signature })
    }
}

const HANDSHAKE_DOMAIN: &[u8] = b"znet-handshake-v1";

pub(crate) fn handshake_transcript(
    initiator_public_key: &[u8; PUBLIC_KEY_LENGTH],
    responder_public_key: &[u8; PUBLIC_KEY_LENGTH],
    initiator_nonce: &Nonce,
    responder_nonce: &Nonce,
    initiator_port: u16,
    responder_port: u16,
) -> Vec<u8> {
    let mut transcript = Vec::new();
    transcript.extend_from_slice(HANDSHAKE_DOMAIN);
    transcript.extend_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    transcript.extend_from_slice(initiator_public_key);
    transcript.extend_from_slice(responder_public_key);
    transcript.extend_from_slice(initiator_nonce);
    transcript.extend_from_slice(responder_nonce);
    transcript.extend_from_slice(&initiator_port.to_be_bytes());
    transcript.extend_from_slice(&responder_port.to_be_bytes());
    transcript
}
