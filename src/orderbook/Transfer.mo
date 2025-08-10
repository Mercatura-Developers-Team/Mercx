import fxmx_icrc1_ledger "canister:fxmx_icrc1_ledger"; // Import fxmx ledger canister
import Debug "mo:base/Debug";
import Result "mo:base/Result";
import Error "mo:base/Error";

module {

  // Type for transfer arguments
  type TransferArgs = {
    amount : Nat;
    toAccount : fxmx_icrc1_ledger.Account; // Use fxmx_icrc1_ledger.Account for fxmx transfers
  };

  // Shared transfer function
  public func transfer(args : TransferArgs) : async Result.Result<fxmx_icrc1_ledger.BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show (args.amount)
      # " tokens to account"
      # debug_show (args.toAccount)
    );

    // Prepare transfer arguments using fxmx_icrc1_ledger's TransferArg
    let transferArgs : fxmx_icrc1_ledger.TransferArg = {
      memo = null;
      amount = args.amount;
      from_subaccount = null; // Optional, depending on the implementation
      fee = null; // Optional, fee can be handled later if needed
      to = args.toAccount; // The account to which we are transferring
      created_at_time = null; // Set to current time if not provided
    };

    try {
      // Call the fxmx_icrc1_ledger's transfer function to transfer tokens
      let transferResult = await fxmx_icrc1_ledger.icrc1_transfer(transferArgs);

      // Check the result of the transfer and return success or error
      switch (transferResult) {
        case (#Err(transferError)) {
          return #err("Couldn't transfer funds:\n" # debug_show (transferError));
        };
        case (#Ok(blockIndex)) {
          return #ok blockIndex; // Return the block index if transfer is successful
        };
      };
    } catch (error : Error) {
      // Catch any errors that might occur during the transfer
      return #err("Reject message: " # Error.message(error));
    };
  };
};