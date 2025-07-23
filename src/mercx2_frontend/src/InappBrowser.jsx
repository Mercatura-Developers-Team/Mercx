import { useState } from "react";
import { CopyToClipboard } from "react-copy-to-clipboard";

export default function InappBrowser() {
  const url = "https://xpm3z-7qaaa-aaaan-qzvlq-cai.icp0.io/";
  const [copied, setCopied] = useState(false);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white px-4">
      <div className="text-center p-6 bg-gray-800 rounded-lg shadow-lg max-w-sm w-full">
        <h2 className="text-lg font-semibold">Enhanced Security Notice</h2>

        <p className="text-gray-300 mt-2 text-sm leading-tight">
          For your protection, <span className="text-blue-400 font-medium">MercX</span> enforces the highest security standards.
        </p>

        <p className="text-gray-300 mt-4 text-sm">
          Please <span className="font-semibold text-blue-400">open this page in Safari or Chrome</span> for a secure experience. 
          Simply copy and paste the link into your browser to ensure a <span className="font-semibold">fully protected connection</span>.
        </p>

        <div className="mt-6">
          <CopyToClipboard
            text={url}
            onCopy={() => {
              setCopied(true);
              setTimeout(() => setCopied(false), 3000); // Reset after 3 seconds
            }}
          >
            <button
              className={`px-6 py-3 text-sm rounded-lg font-medium transition duration-200 ${
                copied ? "bg-blue-500" : "bg-gray-600 hover:bg-gray-700"
              }`}
            >
              {copied ? "Copied!" : "Copy Link"}
            </button>
          </CopyToClipboard>
        </div>

        <p className="mt-6 text-gray-400 text-xs">
          Thank you for trusting <span className="text-blue-400 font-semibold">MercX</span> – Secure, Seamless, Trusted.
        </p>
      </div>
    </div>
  );
}
