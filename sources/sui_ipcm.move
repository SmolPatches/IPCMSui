/*
/// Module: sui_ipcm
module sui_ipcm::sui_ipcm;
*/

// For Move coding conventions, see
// https://docs.sui.io/concepts/sui-move-concepts/conventions
// https://move-book.com/programmability/capability.html
// https://move-book.com/reference/primitive-types/vector.html
// https://docs.sui.io/concepts/sui-move-concepts/conventions
module sui_ipcm::ipcm;
use std::string::{Self,String};
// === Errors ===

// === Structs ===
public struct ReadCap has key, store { id: UID, IPCM_UID: UID } // make sure only owner can read this
public struct WriteCap has key, store { id: UID }
public struct CID has store, copy { // make a custom constructor to make sure its a Content Addressed IPFS thingy
    CID: String // could this be ASCII?
}
public struct IPCM has key, store {
    id: UID,
    CID: 0x0::ipcm::CID, 
}

// only caps can read
public struct PRIV_IPCM has key { // how can i make this private?
    id: UID,
    IPCM: 0x0::ipcm::IPCM,
}

// === Public Functions ===
public fun read(ipcm:&IPCM): CID { // make entry 
    ipcm.CID
}
public fun read_priv(cap:&ReadCap, ipcm:&PRIV_IPCM): CID { // make entry
    //if cap matches
    assert!(&cap.IPCM_UID == &ipcm.id,0); // add better error
    read(&ipcm.IPCM)
}
// === Private Functions ===

// === Events ===
