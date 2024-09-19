import { icrc1_index_canister } from 'declarations/icrc1_index_canister';

export async function fetchTransactions(principalId, maxResults , start) {
    try {
        const response = await icrc1_index_canister.get_account_transactions({
            max_results: maxResults,
       start: start !== undefined ? [start] : null,  // Adjust this if you need to specify a starting point
            account: {
                owner: principalId,
                subaccount: [] // Adjust if subaccounts are used
            }
        });
        
      
        if ('Ok' in response) {
            console.log('Transactions: ', response.Ok.transactions);
            return response.Ok.transactions;
        } else {
            console.error('Error fetching transactions:', response.Err);
            return [];
        }
    } catch (error) {
        console.error('Failed to call the canister:', error);
        throw error;
    }
}
