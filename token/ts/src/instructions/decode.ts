import { u8 } from '@paychains/buffer-layout';
import { TransactionInstruction } from '@paychains/web3.js';
import { TOKEN_PROGRAM_ID } from '../constants';
import { TokenInvalidInstructionDataError, TokenInvalidInstructionTypeError } from '../errors';
import { decodeApproveInstruction, DecodedApproveInstruction } from './approve';
import { decodeApproveCheckedInstruction, DecodedApproveCheckedInstruction } from './approveChecked';
import { decodeBurnInstruction, DecodedBurnInstruction } from './burn';
import { decodeBurnCheckedInstruction, DecodedBurnCheckedInstruction } from './burnChecked';
import { decodeCloseAccountInstruction, DecodedCloseAccountInstruction } from './closeAccount';
import { DecodedFreezeAccountInstruction, decodeFreezeAccountInstruction } from './freezeAccount';
import { DecodedInitializeAccountInstruction, decodeInitializeAccountInstruction } from './initializeAccount';
import { DecodedInitializeMintInstruction, decodeInitializeMintInstruction } from './initializeMint';
import { DecodedInitializeMultisigInstruction, decodeInitializeMultisigInstruction } from './initializeMultisig';
import { DecodedMintToInstruction, decodeMintToInstruction } from './mintTo';
import { DecodedMintToCheckedInstruction, decodeMintToCheckedInstruction } from './mintToChecked';
import { DecodedRevokeInstruction, decodeRevokeInstruction } from './revoke';
import { DecodedSetAuthorityInstruction, decodeSetAuthorityInstruction } from './setAuthority';
import { DecodedSyncNativeInstruction, decodeSyncNativeInstruction } from './syncNative';
import { DecodedThawAccountInstruction, decodeThawAccountInstruction } from './thawAccount';
import { DecodedTransferInstruction, decodeTransferInstruction } from './transfer';
import { DecodedTransferCheckedInstruction, decodeTransferCheckedInstruction } from './transferChecked';
import { TokenInstruction } from './types';

/** TODO: docs */
export function decodeInstruction(
    instruction: TransactionInstruction,
    programId = TOKEN_PROGRAM_ID
):
    | DecodedInitializeMintInstruction
    | DecodedInitializeAccountInstruction
    | DecodedInitializeMultisigInstruction
    | DecodedTransferInstruction
    | DecodedApproveInstruction
    | DecodedRevokeInstruction
    | DecodedSetAuthorityInstruction
    | DecodedMintToInstruction
    | DecodedBurnInstruction
    | DecodedCloseAccountInstruction
    | DecodedFreezeAccountInstruction
    | DecodedThawAccountInstruction
    | DecodedTransferCheckedInstruction
    | DecodedApproveCheckedInstruction
    | DecodedMintToCheckedInstruction
    | DecodedBurnCheckedInstruction
    // | DecodedInitializeAccount2Instruction
    | DecodedSyncNativeInstruction {
    // | DecodedInitializeAccount3Instruction
    // | DecodedInitializeMultisig2Instruction
    // | DecodedInitializeMint2Instruction
    // TODO: implement ^
    if (!instruction.data.length) throw new TokenInvalidInstructionDataError();

    const type = u8().decode(instruction.data);
    if (type === TokenInstruction.InitializeMint) return decodeInitializeMintInstruction(instruction, programId);
    if (type === TokenInstruction.InitializeAccount) return decodeInitializeAccountInstruction(instruction, programId);
    if (type === TokenInstruction.InitializeMultisig)
        return decodeInitializeMultisigInstruction(instruction, programId);
    if (type === TokenInstruction.Transfer) return decodeTransferInstruction(instruction, programId);
    if (type === TokenInstruction.Approve) return decodeApproveInstruction(instruction, programId);
    if (type === TokenInstruction.Revoke) return decodeRevokeInstruction(instruction, programId);
    if (type === TokenInstruction.SetAuthority) return decodeSetAuthorityInstruction(instruction, programId);
    if (type === TokenInstruction.MintTo) return decodeMintToInstruction(instruction, programId);
    if (type === TokenInstruction.Burn) return decodeBurnInstruction(instruction, programId);
    if (type === TokenInstruction.CloseAccount) return decodeCloseAccountInstruction(instruction, programId);
    if (type === TokenInstruction.FreezeAccount) return decodeFreezeAccountInstruction(instruction, programId);
    if (type === TokenInstruction.ThawAccount) return decodeThawAccountInstruction(instruction, programId);
    if (type === TokenInstruction.TransferChecked) return decodeTransferCheckedInstruction(instruction, programId);
    if (type === TokenInstruction.ApproveChecked) return decodeApproveCheckedInstruction(instruction, programId);
    if (type === TokenInstruction.MintToChecked) return decodeMintToCheckedInstruction(instruction, programId);
    if (type === TokenInstruction.BurnChecked) return decodeBurnCheckedInstruction(instruction, programId);
    // TODO: implement
    if (type === TokenInstruction.InitializeAccount2) throw new TokenInvalidInstructionTypeError();
    if (type === TokenInstruction.SyncNative) return decodeSyncNativeInstruction(instruction, programId);
    // TODO: implement
    if (type === TokenInstruction.InitializeAccount3) throw new TokenInvalidInstructionTypeError();
    // TODO: implement
    if (type === TokenInstruction.InitializeMultisig2) throw new TokenInvalidInstructionTypeError();
    // TODO: implement
    if (type === TokenInstruction.InitializeMint2) throw new TokenInvalidInstructionTypeError();

    throw new TokenInvalidInstructionTypeError();
}
