import React, { useEffect, useRef, useState } from "react";
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { orderbook } from "../../declarations/orderbook";  // Correct the path based on your file structure
//import { AuthClient } from "@dfinity/identity"; // No longer needed for now
import { useAuth } from "./use-auth-client";


const decimals = 8;

export default function Orderbook() {
    const [level1, setLevel1] = useState(null);
    const [depth, setDepth] = useState({ bid: [], ask: [] });
    const [result, setResult] = useState("");
    const [tradeHistory, setTradeHistory] = useState([]);

    const txidRef = useRef();
    const quantityRef = useRef();
    const priceRef = useRef();
    const unitSizeRef = useRef();
    const orderTypeRef = useRef();
   const{principal}=useAuth();
     // ✅ DEFINE full userAccount object
  const userAccount = {
    owner: principal,
    subaccount: []
  };
    useEffect(() => {
        const update = async () => {
            try {
                const l1 = await orderbook.level1(); // Fetch the level 1 prices
                setLevel1(l1);
                const d = await orderbook.depth([]); // Fetch the depth of the order book
                setDepth(d);
            } catch (err) {
                console.error("Fetch failed", err);
            }
        };

        update();
        const interval = setInterval(update, 5000);
        return () => clearInterval(interval);
    }, []);

    // Internet Identity Authentication - Connect and set principal
    // Skipping for now as it's not needed

    const handleTrade = async (side) => {
        const txid = txidRef.current.value.trim();
        const quantity = Math.floor(Number(quantityRef.current.value) * 10 ** decimals);
        const price = Math.floor(Number(priceRef.current.value) * 10 ** decimals);
        const unitSize = Math.floor(Number(unitSizeRef.current.value) * 10 ** decimals);
        const orderType = orderTypeRef.current.value;

        if (!txid || quantity <= 0 || price <= 0 || unitSize <= 0) {
            setResult("⚠️ Please fill all fields correctly.");
            return;
        }

        try {
            const encodedTxid = Array.from(new TextEncoder().encode(txid));
            const sideVariant = side === "Buy" ? { Buy: null } : { Sell: null };
            const orderTypeVariant = orderType === "Limit" ? { LMT: null } : { MKT: null };

            // Skip principal check for now - make sure you pass the principal later once it's connected via II or other methods.
            const res = await orderbook.orchestrateTrade(
                encodedTxid,
                sideVariant,
                quantity,
             price,
            orderTypeVariant,
            unitSize,
            userAccount // Pass the principal or use anonymous for now
                
            );
console.log("Trade Params", {
  sideVariant,
  orderTypeVariant,
  encodedTxid,
  quantity,
  price,
  unitSize,
  userAccount,
});
            if ("ok" in res) {
                setResult(`✅ Trade complete: ${res.ok.length} fills`);
                const history = [...tradeHistory];
                res.ok.forEach(fill => {
                    const fillQty = Number(fill.token0Value.CreditRecord || fill.token0Value.DebitRecord || 0);
                    const fillAmt = Number(fill.token1Value.CreditRecord || fill.token1Value.DebitRecord || 0);
                    const avgPrice = (fillAmt / fillQty).toFixed(2);
                    history.unshift(`${side} ${fillQty / 10 ** decimals} FXMX @ ${avgPrice} USD`);
                });
                setTradeHistory(history.slice(0, 10));
            } else {
                setResult("❌ Error: " + res.err);
            }
        } catch (err) {
            setResult("❌ Error: " + err.message);
        }
    };

    return (
        <div className="p-6 text-white bg-[#0f0f23]">
            <h2 className="text-2xl font-bold mb-4">📊 FXMX Orderbook</h2>

            <div className="grid grid-cols-2 gap-6">
                <div>
                    <h3 className="font-semibold">Level 1</h3>
                    <table className="table-auto border w-full text-sm">
                        <tbody>
                            {level1 && (
                                <>
                                    <tr className="text-green-400">
                                        <td>Buy</td>
                                        <td>{Number(level1.bestBid.price) / 10 ** decimals}</td>
                                        <td>{Number(level1.bestBid.quantity) / 10 ** decimals}</td>
                                    </tr>
                                    <tr className="text-red-400">
                                        <td>Sell</td>
                                        <td>{Number(level1.bestAsk.price) / 10 ** decimals}</td>
                                        <td>{Number(level1.bestAsk.quantity) / 10 ** decimals}</td>
                                    </tr>
                                </>
                            )}
                        </tbody>
                    </table>

                    <h3 className="mt-4 font-semibold">Depth</h3>
                    <table className="table-auto border w-full text-sm">
                        <tbody>
                            {depth.bid.map((b, i) => (
                                <tr key={i} className="text-green-400">
                                    <td>Buy</td>
                                    <td>{Number(b.price) / 10 ** decimals}</td>
                                    <td>{Number(b.quantity) / 10 ** decimals}</td>
                                </tr>
                            ))}
                            {depth.ask.map((a, i) => (
                                <tr key={i} className="text-red-400">
                                    <td>Sell</td>
                                    <td>{Number(a.price) / 10 ** decimals}</td>
                                    <td>{Number(a.quantity) / 10 ** decimals}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>

                <div>
                    <h3 className="font-semibold">Trade</h3>
                    <form onSubmit={(e) => e.preventDefault()} className="space-y-2">
                        <input ref={txidRef} placeholder="TxID" className="bg-gray-800 p-2 w-full" />
                        <input ref={quantityRef} placeholder="Quantity" className="bg-gray-800 p-2 w-full" />
                        <input ref={priceRef} placeholder="Price" className="bg-gray-800 p-2 w-full" />
                        <input ref={unitSizeRef} placeholder="Unit Size" className="bg-gray-800 p-2 w-full" />
                        <select ref={orderTypeRef} className="bg-gray-800 p-2 w-full">
                            <option value="Limit">Limit</option>
                            <option value="IOC">IOC</option>
                        </select>
                        <div className="flex gap-2">
                            <button onClick={() => handleTrade("Buy")} className="bg-green-600 px-4 py-2">Buy</button>
                            <button onClick={() => handleTrade("Sell")} className="bg-red-600 px-4 py-2">Sell</button>
                        </div>
                    </form>
                    <p className="mt-2">{result}</p>

                    <h4 className="mt-4 font-semibold">Trade History</h4>
                    <ul className="text-sm list-disc pl-4">
                        {tradeHistory.map((t, i) => <li key={i}>{t}</li>)}
                    </ul>
                </div>
            </div>
        </div>
    );
}