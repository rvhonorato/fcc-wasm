import React from 'react';
import './App.css';
import init, { add, decode_base64 } from 'wasm-lib';
import { decode } from 'punycode';

function App() {
  const [ans, setAns] = React.useState(0);
  const [fileData, setFileData] = React.useState<string | null>(null);

  let x = 1;
  let y = 1;

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
          // init().then(() => {
          //   let v = decode_base64(res as string);
          //   console.log("File size", v);
          // });
          setFileData(res as string);
        });
    };
    init().then(() => {
      // let result = add(x, y);
      // let v = add(x, y);
      if (fileData) {
        let v = decode_base64(fileData as string);
        console.log("File size", v);
      }
      // setAns(result);
      // console.log("File size", v);
    });

    // console.log("File uploaded", e.target.files);
    // let size = decode_base64(fileData as string);
    // console.log("File size", v);
  };

  // Do it as soon as the component is mounted
  // React.useEffect(() => {
  //   init().then((res) => {
  //     let result = add(x, y);
  //     setAns(result);
  //   });
  // }, []);
  return (
    <div className="App">
      <input type="file" onChange={handleFileChange} />
      <p>
        {fileData}
      </p>
    </div>
  );
}



export default App;
