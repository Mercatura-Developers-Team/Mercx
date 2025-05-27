import Principal "mo:base/Principal";
import Blob "mo:base/Blob";
import Nat "mo:base/Nat";
import Trie "mo:base/Trie";
import OrderBook "./OrderBook";
import fxmx_icrc1_ledger "canister:fxmx_icrc1_ledger"; // Import fxmx ledger canister
import Debug "mo:base/Debug";
import Result "mo:base/Result";
import Error "mo:base/Error";
import tommy_icrc1_ledger "canister:tommy_icrc1_ledger"; // Import Tommy ledger canister

actor Test {


    // Aliases for order types and KLine data
    type Account = fxmx_icrc1_ledger.Account; // Use fxmx_icrc1_ledger.Account for fxmx transfers
    type TommyAccount = tommy_icrc1_ledger.Account; // Account type for Tommy token (new type)

  type TransferArgs = {
        amount : Nat;
        toAccount : fxmx_icrc1_ledger.Account; // Use fxmx_icrc1_ledger.Account for fxmx transfers
    };



      // Tommy Transfer Arguments Type (Updated)
    type TommyTransferArgs = {
        amount : Nat;  // Amount of Tommy tokens to transfer
        toAccount : tommy_icrc1_ledger.Account;  // Fully qualified TommyAccount for recipient
    };

    // Tommy Approve Arguments Type (Updated)
    type TommyApproveArgs = {
        fee : ?Nat;  // Optional fee
        from : tommy_icrc1_ledger.Account;  // TommyAccount for sender
        memo : ?Blob;  // Optional memo
        created_at_time : ?Nat64;  // Timestamp for approval creation
        amount : Nat;  // Amount of Tommy tokens to approve
        expected_allowance : ?Nat;  // Expected allowance (optional)
        expires_at : ?Nat64;  // Expiration timestamp (optional)
        spender : tommy_icrc1_ledger.Account;  // TommyAccount for spender
    };

    // Tommy Transfer From Arguments Type (Updated)
    type TommyTransferFromArgs = {
        spender_subaccount : ?Blob;  // Optional subaccount for spender
        from : tommy_icrc1_ledger.Account;  // TommyAccount for sender
        to : tommy_icrc1_ledger.Account;  // TommyAccount recipient
        amount : Nat;  // Amount of Tommy tokens to transfer
        fee : ?Nat;  // Optional fee
        memo : ?Blob;  // Optional memo
        created_at_time : ?Nat64;  // Timestamp of the transfer
    };

    type Tokens = Nat;

    type Approve = {
        fee : ?Nat;                  // Optional fee for the transaction
        from : Account;              // The account granting approval
        memo : ?Blob;                // Optional memo for the transaction
        created_at_time : ?Nat64;    // Optional timestamp when approval was created
        amount : Nat;                // Amount of tokens approved for transfer
        expected_allowance : ?Nat;   // Expected allowance (optional)
        expires_at : ?Nat64;        // Expiration timestamp (optional)
        spender : Account;           // The account allowed to spend tokens
    };

    type ApproveArgs = {
        fee : ?Nat;                  // Optional fee for the approval
        memo : ?Blob;                // Optional memo
        from_subaccount : ?Blob;     // Optional subaccount for the sender
        created_at_time : ?Nat64;    // Optional timestamp of creation
        amount : Nat;                // The amount of tokens to approve
        expected_allowance : ?Nat;   // Expected allowance for the spender
        expires_at : ?Nat64;        // Expiration timestamp (optional)
        spender : Account;           // The account that is approved to spend tokens
    };

    type TransferFromArgs = {
        spender_subaccount : ?Blob;  // Optional subaccount for the spender
        from : Account;              // The account from which tokens will be transferred
        to : Account;                // The recipient of the tokens
        amount : Nat;                // The amount of tokens to transfer
        fee : ?Nat;                  // Optional fee for the transfer
        memo : ?Blob;                // Optional memo for the transfer
        created_at_time : ?Nat64;    // Optional timestamp for the transfer
    };




    type Quantity = Nat; // Use Nat for quantity
    type Timestamp = Nat64; // Alias Timestamp to Nat64 if needed    
    type OrderSide = OrderBook.OrderSide;
    type OrderType = OrderBook.OrderType;
    type OrderPrice = OrderBook.OrderPrice;
    type Tick = OrderBook.Tick;
    type OrderFilled = OrderBook.OrderFilled;
    type KBar = OrderBook.KBar;
    type KInterval = OrderBook.KInterval;
    type KLines = OrderBook.KLines;

  
    // Stable state: orderbook + chart data
    private stable var ic_orderBook: OrderBook.OrderBook = OrderBook.create();
    private stable var klines: KLines = OrderBook.createK(); // <-- ✅ charting data




public shared func orchestrateTradeTommy(
    txid: Blob,
    side: OrderSide,
    quantity: Nat,  // Ensure 'quantity' is explicitly declared as 'Nat'
    price: Nat,
    orderType: OrderType,
    unitSize: Nat,
    userAccount: tommy_icrc1_ledger.Account  // Use 'tommy_icrc1_ledger.Account' for Tommy transfers
) : async Result.Result<(), Text> {

    // Step 1: Check balance for Tommy tokens
    let balanceResult = await checkBalanceTommy(userAccount);  // Using Tommy's checkBalance function
    switch (balanceResult) {
        case (#err(errorMessage)) {
            return #err("Balance check failed: " # errorMessage); // If balance check fails, return error
        };
        case (#ok(userBalance)) {
            if (userBalance < quantity) {
                return #err("Insufficient balance"); // If balance is not enough, return error
            };
        };
    };

    // Step 2: Transfer Tommy tokens to backend
    let transferArgsTommy : tommy_icrc1_ledger.TransferArg = {  
        amount = quantity;  // Correct: 'quantity' is passed as a 'Nat' and is equivalent to 'Tokens'
        created_at_time = null;     // Optional: You can use Timestamp.now() if you need the current time
        fee = null;                 // Optional: Set to null if not used
        from_subaccount = null;     // Optional: Set to null if not using subaccounts
        memo = null;                // Optional: Set to null if not using memo
        to = userAccount;           // Correct: Use 'userAccount' (type 'tommy_icrc1_ledger.Account')
    };

    // Call Tommy ledger's transfer function
    let transferResult = await tommy_icrc1_ledger.icrc1_transfer(transferArgsTommy);  
    switch (transferResult) {
        case (#err(errorMessage)) {
            return #err("Transfer to backend failed: " # errorMessage); // Handle transfer failure
        };
        case (#ok(blockIndex)) {
            Debug.print("Transfer from Tommy ledger to backend successful: " # debug_show(blockIndex)); // Handle transfer success
        };
        case (_) {
            return #err("Unexpected result received during transfer"); // Default case for unexpected results
        };
    };

    // Step 3: Execute the trade
    let tradeResult = await trade(txid, side, quantity, price, orderType, unitSize);  // Calling the trade function
    switch (tradeResult) {
        case (#err(errorMessage)) {
            return #err("Trade failed: " # errorMessage); // Handle trade failure
        };
        case (#ok(filledOrders)) {
            Debug.print("Trade successful: " # debug_show(filledOrders));

         // Step 4: Transfer Tommy tokens back to user after trade
let transferBackArgsTommy : tommy_icrc1_ledger.TransferArg = {  
    amount = quantity;  // Correctly pass 'quantity' (of type 'Nat') as 'Tokens'
    to = userAccount;   // Correct: Use 'userAccount' (type 'tommy_icrc1_ledger.Account') for the recipient
    created_at_time = null;   // Optional: Timestamp, can be set to null or Timestamp.now()
    fee = null;               // Optional: Fee, can be set to null
    from_subaccount = null;   // Optional: Set to null if not using subaccounts
    memo = null;              // Optional: Set to null if not using memo
};

// Call Tommy ledger's transfer function again
let transferBackResult = await tommy_icrc1_ledger.icrc1_transfer(transferBackArgsTommy);  
switch (transferBackResult) {
    case (#Err(error)) {
        // Handle different error types in the TransferError
        let errorMessage = switch error {
            case (#BadBurn(min_burn_amount)) { "Bad burn: " # debug_show(min_burn_amount) };
            case (#BadFee(expected_fee)) { "Bad fee: " # debug_show(expected_fee) };
            case (#CreatedInFuture(ledger_time)) { "Created in future: " # debug_show(ledger_time) };
            case (#Duplicate(duplicate_of)) { "Duplicate: " # debug_show(duplicate_of) };
            case (#InsufficientFunds(balance)) { "Insufficient funds: " # debug_show(balance) };
            case (#TemporarilyUnavailable) { "Temporarily unavailable" };
            case (#TooOld) { "Too old" };
        };
        return #err("Transfer from backend to user failed: " # errorMessage); // Handle transfer failure with the error message
    };
    case (#Ok(blockIndex)) {
        Debug.print("Transfer from backend to user successful: " # debug_show(blockIndex)); // Handle transfer success
    };
    case (_) {
        return #err("Unexpected result received during transfer"); // Default case for unexpected results
    };

            };
        };
    };

    return #ok(()); // Return success if all steps are completed successfully
};



public shared func orchestrateTrade(
    txid: Blob,
    side: OrderSide,
    quantity: Nat,  // Ensure 'quantity' is explicitly declared as 'Nat'
    price: Nat,
    orderType: OrderType,
    unitSize: Nat,
    userAccount: fxmx_icrc1_ledger.Account  // Ensure 'userAccount' is explicitly declared
) : async Result.Result<(), Text> {

    // Step 1: Check balance

    let balanceResult = await checkBalance(userAccount);
    switch (balanceResult) {
        case (#err(errorMessage)) {
            return #err("Balance check failed: " # errorMessage); // If balance check fails, return error
        };
        case (#ok(userBalance)) {
            if (userBalance < quantity) {
                return #err("Insufficient balance"); // If balance is not enough, return error
            };
        };
    };

    // Step 2: Transfer tokens to backend
    let transferArgs : TransferArgs = {  // Declare 'transferArgs' of type 'TransferArgs'
        amount = quantity;  // Correct: using 'quantity' (of type 'Nat')
        toAccount = userAccount// Correct: 'userAccount' (of type 'fxmx_icrc1_ledger.Account')
    };

    let transferResult = await transfer(transferArgs);  // Calling the transfer function
    switch (transferResult) {
        case (#err(errorMessage)) {
            return #err("Transfer to backend failed: " # errorMessage); // Handle error if transfer fails
        };
        case (#ok(blockIndex)) {
            Debug.print("Transfer from FXMX ledger to backend successful: " # debug_show(blockIndex));
        };
    };

    // Step 3: Execute the trade
    let tradeResult = await trade(txid, side, quantity, price, orderType, unitSize);  // Calling the trade function
    switch (tradeResult) {
        case (#err(errorMessage)) {
            return #err("Trade failed: " # errorMessage); // Handle trade failure
        };
        case (#ok(filledOrders)) {
            Debug.print("Trade successful: " # debug_show(filledOrders));

            // Step 4: Transfer tokens back to user after trade
            let transferBackArgs : TransferArgs = {  // Declare 'transferBackArgs' of type 'TransferArgs'
                amount = quantity; // Correct: 'quantity' (of type 'Nat')
                toAccount = userAccount// Correct: 'userAccount' (of type 'fxmx_icrc1_ledger.Account')
            };

            let transferBackResult = await transfer(transferBackArgs);  // Calling the transfer function again
            switch (transferBackResult) {
                case (#err(errorMessage)) {
                    return #err("Transfer from backend to user failed: " # errorMessage); // Handle error
                };
                case (#ok(blockIndex)) {
                    Debug.print("Transfer from backend to user successful: " # debug_show(blockIndex));
                };
            };
        };
    };

    return #ok(()); // Return success if all steps are completed successfully
};



// Tommy token balance check function
public shared func checkBalanceTommy(account: tommy_icrc1_ledger.Account) : async Result.Result<tommy_icrc1_ledger.Tokens, Text> {
    try {
        // Call Tommy ledger canister's balance_of function
        let balanceResult = await tommy_icrc1_ledger.icrc1_balance_of(account);
        return #ok(balanceResult); // Return the balance if successful
    } catch (error : Error) {
        return #err("Failed to fetch balance for Tommy token: " # Error.message(error)); // Handle errors
    };
};

// Tommy token transfer function
public shared func icrc2_transfer_from_tommy(
    spender: tommy_icrc1_ledger.Account,   // The account performing the transfer (the approved spender)
    from: tommy_icrc1_ledger.Account,      // The account from which tokens are transferred
    to: tommy_icrc1_ledger.Account,        // The account to which tokens are transferred
    amount: tommy_icrc1_ledger.Tokens,     // The amount of tokens to transfer (Tommy-specific Tokens)
    fee: ?tommy_icrc1_ledger.Tokens,       // Optional fee for the transfer
    memo: ?Blob,              // Optional memo for the transaction
    created_at_time: ?Timestamp // Optional timestamp of when the transfer is made
) : async Result.Result<(), Text> {
    // Step 1: Call Tommy ledger canister's transfer function
    let transferArgsTommy = {
        from_subaccount = null;         // Optional subaccount for the sender
        to = to;                        // The account to which tokens will be transferred
        amount = amount;                // The amount to transfer
        fee = fee;                      // Optional fee
        memo = memo;                    // Optional memo
        created_at_time = created_at_time; // Optional timestamp
    };

    // Step 2: Execute the transfer operation using Tommy ledger
    let transferResultTommy = await tommy_icrc1_ledger.icrc1_transfer(transferArgsTommy);

    // Step 3: Check the result of the transfer
    switch (transferResultTommy) {
        case (#Err(transferError)) {
            // If the transfer failed, return an error
            return #err("Transfer failed for Tommy token: " # debug_show(transferError));
        };
        case (#Ok(blockIndex)) {
            // If the transfer was successful, return success
            return #ok(());
        };
    };
};

    /// MAIN TRADE FUNCTION
    public shared func trade(
        txid: Blob,
        side: OrderSide,
        quantity: Nat,
        price: Nat,
        orderType: OrderType,
        unitSize: Nat
    ) : async Result.Result<[OrderFilled], Text> {
        let incoming: OrderPrice = switch side {
            case (#Buy) { { quantity = #Buy((quantity, quantity * price / unitSize)); price = price } };
            case (#Sell) { { quantity = #Sell(quantity); price = price } };
        };

        let result = OrderBook.trade(ic_orderBook, txid, incoming, orderType, unitSize);

        // Save updated book
        ic_orderBook := result.ob;

        // Save K-line (chart) updates
        klines := OrderBook.putBatch(klines, result.filled, unitSize);

        return #ok(result.filled); // ✅ Now returns all filled trades
    };

    /// Top-of-book prices
    public query func level1() : async Tick {
        return OrderBook.level1(ic_orderBook);
    };

    /// Full orderbook depth (useful for UI)
    public query func depth(depth: ?Nat) : async {
        ask: [OrderBook.PriceResponse];
        bid: [OrderBook.PriceResponse];
    } {
        return OrderBook.depth(ic_orderBook, depth);
    };

    /// Clear the orderbook (admin-only)
    public shared func clearOrderBook() : async () {
        ic_orderBook := OrderBook.clear(ic_orderBook);
    };

    /// Check if specific order exists
    public query func inOrderBook(txid: Blob) : async Bool {
        return OrderBook.inOrderBook(ic_orderBook, txid);
    };

    /// Get specific order
    public query func getOrder(txid: Blob, side: ?OrderSide) : async ?OrderPrice {
        return OrderBook.get(ic_orderBook, txid, side);
    };

    /// Remove an order manually
    public shared func removeOrder(txid: Blob, side: ?OrderSide) : async () {
        ic_orderBook := OrderBook.remove(ic_orderBook, txid, side);
    };

    /// 📊 Return K-line chart data for selected interval
    public query func getK(interval: KInterval) : async [KBar] {
        return OrderBook.getK(klines, interval);
    };

    public shared func transfer(args : TransferArgs) : async Result.Result<fxmx_icrc1_ledger.BlockIndex, Text> {
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




// Tommy token transfer function
public shared func transferTommy(args : TommyTransferArgs) : async Result.Result<tommy_icrc1_ledger.BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show(args.amount)
      # " Tommy tokens to account"
      # debug_show(args.toAccount)
    );

    // Prepare transfer arguments using Tommy's TransferArg
    let transferArgsTommy : tommy_icrc1_ledger.TransferArg = {
        memo = null;  // Optional memo for the transfer
        amount = args.amount;  // The amount to transfer
        from_subaccount = null; // Optional subaccount for the sender
        fee = null;  // Optional fee, could be added later
        to = args.toAccount;  // The account to which Tommy tokens will be transferred
        created_at_time = null;  // Optional timestamp
    };

    try {
        // Call the Tommy ledger canister's icrc1_transfer function to transfer Tommy tokens
        let transferResultTommy = await tommy_icrc1_ledger.icrc1_transfer(transferArgsTommy);

        // Check the result of the transfer and return success or error
        switch (transferResultTommy) {
            case (#Err(transferError)) {
                return #err("Couldn't transfer Tommy tokens:\n" # debug_show(transferError));  // Handle transfer failure
            };
            case (#Ok(blockIndex)) {
                return #ok(blockIndex);  // Return the block index if transfer is successful
            };
        };
    } catch (error : Error) {
        // Catch any errors that might occur during the transfer
        return #err("Reject message: " # Error.message(error));  // Handle transfer failure
    };
};


// Define the function to check balance for fxmx tokens
public func checkBalance(account: fxmx_icrc1_ledger.Account) : async Result.Result<fxmx_icrc1_ledger.Tokens, Text> {
    try {
        // Query the fxmx ledger canister's balance_of function
        let balanceResult = await fxmx_icrc1_ledger.icrc1_balance_of(account);
        
        // If successful, return the balance
        return #ok(balanceResult);
    } catch (error : Error) {
        // Handle any errors that might occur
        return #err("Failed to fetch balance for fxmx token: " # Error.message(error));
    };
};


};
