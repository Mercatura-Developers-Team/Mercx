import React from 'react';
import QRCode from 'react-qr-code';
import { Principal } from '@dfinity/principal';
import { CopyToClipboard } from 'react-copy-to-clipboard';

const ReceiveModal = ({ principal, onClose }) => {
  const principalText = principal.toString();
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-slate-800 rounded-lg p-6 max-w-md w-full">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-xl font-bold text-white">Receive Tokens</h3>
          <button 
            onClick={onClose}
            className="text-gray-400 hover:text-white"
          >
            ✕
          </button>
        </div>
        
        <div className="flex flex-col items-center gap-4">
          <div className="p-4 bg-white rounded-lg">
            <QRCode 
              value={principalText} 
              size={128}
              level="H"
              includeMargin={true}
            />
          </div>
          
          <div className="w-full">
            <p className="text-gray-400 mb-2 text-sm">Your Principal ID:</p>
            <div className="flex items-center gap-2 bg-slate-700 p-2 rounded">
              <code className="text-white text-sm overflow-x-auto flex-1">
                {principalText}
              </code>
              <CopyToClipboard text={principalText}>
                <button className="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-1 rounded text-sm">
                  Copy
                </button>
              </CopyToClipboard>
            </div>
          </div>
          
          <button
            onClick={onClose}
            className="mt-4 bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-lg"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default ReceiveModal;