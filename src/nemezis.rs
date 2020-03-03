use std::io::Write;

#[cfg(not(feature = "quantum"))]
pub fn generate_nemezis_block(keys: &crate::pk::PetKey) -> (crate::block::Block, crate::transaction::Transaction){
    let ConsensusSettings = crate::conset::ConsensusSettings::default();
    let nemezis_body = crate::transaction::TxBody::new([0;32], serde_json::to_vec(&ConsensusSettings).unwrap()); 
    let nemesis_tx = crate::transaction::Transaction::new(nemezis_body, &keys.ec);
    let mut nemezis_vec = Vec::new();
    nemezis_vec.push(nemesis_tx.hash());
    let block = crate::block::Block::new(hex::encode([0;32]), nemezis_vec, &keys.ec, 0);
    let mut pemf = std::fs::File::create(std::path::Path::new("NEMEZIS")).unwrap();
    pemf.write_all(&serde_json::to_vec(&block).unwrap());
    (block, nemesis_tx)
}

#[cfg(feature = "quantum")]
pub fn generate_nemezis_block(keys: &crate::pk::PetKey) -> (crate::block::Block, crate::transaction::Transaction){
    let ConsensusSettings = crate::conset::ConsensusSettings::default();
    let nemezis_body = crate::transaction::TxBody::new([0;32], serde_json::to_vec(&ConsensusSettings).unwrap()); 
    let nemesis_tx = crate::transaction::Transaction::new(nemezis_body, &keys.glp);
    let mut nemezis_vec = Vec::new();
    nemezis_vec.push(nemesis_tx.hash());
    let block = crate::block::Block::new([0;32], nemezis_vec, &keys.glp, 0);
    let mut pemf = std::fs::File::create(std::path::Path::new("qNEMEZIS")).unwrap();
    pemf.write_all(&serde_json::to_vec(&block).unwrap());
    (block, nemesis_tx)
}