import { useState } from "react";
import { CopyToClipboard } from 'react-copy-to-clipboard';
export default function InappBrowser() {
  const url = "https://xpm3z-7qaaa-aaaan-qzvlq-cai.icp0.io/";
  const [copied, setCopied] = useState(false);

  // const copyToClipboard = () => {
  //   navigator.clipboard.writeText(url).then(() => {
  //     setCopied(true);
  //     setTimeout(() => setCopied(false), 2000); // Reset after 2 sec
  //   });
  // };

  return (
    <div style={{ textAlign: "center", marginTop: "50px", fontFamily: "Arial, sans-serif" }}>
      <h2>Open This Page in Safari or Chrome</h2>
      <p>To continue, open this page in your default browser:</p>



      <div>



        <CopyToClipboard text={url} onCopy={() => {
          setCopied(true);
          setTimeout(() => {
            setCopied(false);
          }, 3000);
        }}>

          <button
            style={{
              background: copied ? "#007bff" : "#6c757d",

              color: "white",
              padding: "10px 20px",
              border: "none",
              cursor: "pointer",

            }}

            className="bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker text-white font-bold text-lg  py-2 px-4 rounded"
          >
            {copied ? "Copied!" : "Copy Link"}
          </button>

        </CopyToClipboard>

      </div>
    </div>
  );
}
