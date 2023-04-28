import React from 'react';
import './App.css';
import init, { fcc } from 'wasm-lib';

function App() {
  const [fileData, setFileData] = React.useState<string | null>(null);
  const [clusterOut, setClusterOut] = React.useState<string | null>(null);


  // Read a file as base64
  const readFileAsBase64 = (file: File) => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        const res = reader.result;
        if (typeof res === 'string') {
          resolve(res);
        } else {
          reject('Failed to read the file');
        }
      };
      reader.readAsDataURL(file);
    });
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    console.log("File uploaded", e.target.files);
    if (e.target.files) {
      readFileAsBase64(e.target.files[0])
        .then((res) => {
          setFileData(res as string);
        });
    };
    // console.log("File uploaded", e.target.files);
  };

  const runWasm = () => {
    init().then(() => {
      let v = fcc(fileData as string);
      setClusterOut(v);
      // console.log("File size", v);
    });
  };

  return (
    <div className="App">
      <input type="file" onChange={handleFileChange} />
      <br />
      <button onClick={runWasm}>Run FCC</button>
      <pre>
        {clusterOut}
      </pre>
    </div>
  );
}



export default App;
