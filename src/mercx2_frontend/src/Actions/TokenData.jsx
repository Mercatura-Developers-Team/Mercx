import React from 'react';


const TokenData = ({TokenBalance, TokenName,TokenLogo}) => {


    return (<>  <div className="flex justify-between items-center">
        <label
            className="flex items-center px-2 py-2 text-sm text-gray-300 shadow-md rounded-md hover:bg-slate-700"
        >
            <img
                src={TokenLogo}
                alt=""
                height={20}
                width={20}
                layout="fixed"
            />
            <span className="font-bold mx-2">
            {TokenName}
            </span>
          
        </label>
        <p 
        className="text-gray-300 text-sm font-medium">
            Balance: {TokenBalance}
        </p>

    </div>
    </>);
}

export default TokenData;