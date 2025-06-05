
// For Move coding conventions, see
// https://docs.sui.io/concepts/sui-move-concepts/conventions
// https://move-book.com/programmability/capability.html
// https://move-book.com/reference/primitive-types/vector.html
// https://docs.sui.io/concepts/sui-move-concepts/conventions
module sui_ipcm::ipcm;
use std::string::{String};
use sui::event;
// === Errors ===

#[error]
const EInvadlidPerm: vector<u8> = b"Invalid Permissions";

// === Structs ===

/// CID type, made to logically signify an IPFS CID
/// Note that SUI CID's are varied and enforcment of type validity should be done by RPC calling code off chain
public struct CID has store, copy, drop{ 
    cid: vector<u8>,
    desc: Option<String>
}

/// Module: sui_ipcm
/// Allow Pinning of "CID(s)" on Sui
/// In reality since CIDs need to be verfied off chain before pushing
/// People can push other objects like GPG identities and IPFS content
public struct IPCM has key, store {
    id: UID,
    cid: CID,
    owner: address
}


// === Public Functions ===
public fun read(ipcm:&IPCM): CID { 
    ipcm.cid
}
public fun makeCID(cid:vector<u8>,desc:Option<String>): CID {
    CID {
	cid,
	desc
    }
}
// === Private Functions ===

// === Events ===

// Entry
public entry fun mint(cid: vector<u8>,desc:Option<String>,ctx:&mut TxContext) {
    let cid = makeCID(cid,desc);
    let id = object::new(ctx);
    let owner = tx_context::sender(ctx);
    transfer::transfer(
	IPCM {
	    id,
	    cid,
	    owner,
	},
        tx_context::sender(ctx)
    );
}
public struct UpdatedIPCM has copy, drop{
    ipcm_id: ID
}
public entry fun update(ipcm:&mut IPCM,cid:vector<u8>,desc:Option<String>,update_desc: bool,ctx:&mut TxContext) {
    let cid = makeCID(cid,desc);
    assert!(ipcm.owner == tx_context::sender(ctx),EInvadlidPerm);
    if (update_desc) { 
	ipcm.cid = cid;
	return
    };
    ipcm.cid.cid = cid.cid;
    event::emit(UpdatedIPCM {
	ipcm_id: object::id(ipcm)
    });
}
public entry fun read_ipcm(ipcm:&IPCM,_:&mut TxContext): CID {
    read(ipcm)
}
