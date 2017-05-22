#![allow(dead_code)]

use chain::Address;

pub struct ArtifactEntry {
	signature: [u8; 32],
	body: ArtifactHeader
}

pub struct ArtifactHeader {
	timestamp: u64,
	author: Address,
	contents: ArtSpec
}

pub enum ArtSpec {
	Unknown { ver: u16, raw: Vec<u8> },
	RootIdent { art: IdentityDeclararionArtifact }, // Should this really just be the same?
	IdentDecl { art: IdentityDeclararionArtifact },
	SimpleTextMsg { art: SimpleTextMessageArtifact }
}

impl ArtSpec {

	#[allow(unused_variables)]
	fn get_version(self) -> u16 {

		match self {
			ArtSpec::Unknown { ver, raw } => ver,
			ArtSpec::RootIdent { art } => 0x0000,
			ArtSpec::IdentDecl { art } => 0x0001,
			ArtSpec::SimpleTextMsg { art } => 0x0010
		}

	}

}

pub struct IdentityDeclararionArtifact {
	key: Vec<u8>
}

pub struct SimpleTextMessageArtifact {
	msg: Vec<u8>
}
