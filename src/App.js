import logo from './logo.svg';
import './App.css';

import { useEffect, useState } from 'react'
import React from 'react'


function App() {
  const [response, setResponse] = useState("")
  const [identityPath, setIdentityPath] = useState("")
  const [hasPath, setHasPath] = useState(false)
  const [sendFundsResponse, setSendFundsResponse] = useState("")

  const pathRef = React.useRef();
  const amountRef = React.useRef();
  const sendToRef = React.useRef();

  // // Ask the backend to load the identities
  // useEffect(() => {
  //   window.__TAURI__.invoke("load_identities", { argument: "/home/devel/.config/dfx/identity" })
  //     .then((response) => { setResponse(response) })
  //     .catch((error) => { setResponse(error) })
  // }, []);


  const handleWalletSend = (e) => {
    e.preventDefault();

    const amount = amountRef.current.value.toString();
    const sendTo = sendToRef.current.value.toString();

    window.__TAURI__.invoke("cmd_send_funds", { sendAmount: amount, sendTo: sendTo })
      .then((response) => {
        setSendFundsResponse(response);
      })
      .catch((error) => { setSendFundsResponse(error) })

  }

  const handlePath = (e) => {
    e.preventDefault();

    const path = pathRef.current.value.toString();

    // console.log(path)
    window.__TAURI__.invoke("set_dfx_identity_path", { argument: path })
      .then((response) => {
        setResponse(response);
        setIdentityPath(path);
        setHasPath(true)
      })
      .catch((error) => { setResponse(error) })



  }

  const handleCommand = (e) => {
    e.preventDefault();

    window.__TAURI__.invoke("cmd_refresh_wallet", { argument: "" })
      .then((response) => {
        setResponse(response);
        console.log(response)
      })
      .catch((error) => { setResponse(error) })

  }

  return (
    < div >
      <div className="box has-text-centered">
        <section className="section">

          <div className="has-text-weight-semibold">
            DevPouch Wallet
          </div>
        </section>
        {
          hasPath === false &&
          <div>
            <div>
              Please set the path to your dfx identity.pem file (absolute path)
            </div>

            <form onSubmit={handlePath}>
              <input type="text" name="path" ref={pathRef}></input>
              <button id="handlePath" type="submit">Save</button>
            </form>
          </div>
        }
        {
          hasPath === true &&
          <>
            <div>
              <div className="has-text-primary is-size-7">
                Your path is set to {identityPath}
              </div>
            </div>

            <form onSubmit={handleCommand}>
              <button className="button is-primary" id="submit" type="submit">Refresh Wallet</button>
            </form>
            <div> {
              response.hasOwnProperty('balance') &&
              <>
                <div><strong>Balance: </strong>{response.balance.toString()}</div>
                <div><strong>Principal: </strong> {response.principal.toString()}</div>
                <div><strong>Account Identifier: </strong> {response.account_identifier.toString()}</div>
              </>
            } </div>

            <section className="section">
              <div className="has-text-weight-semibold">Send ICP</div>

              <div>
                <form onSubmit={handleWalletSend}>
                  <div>
                    Amount: <input type="text" name="path" ref={amountRef}></input>
                  </div>
                  <div>
                    Destination: <input type="text" name="path" ref={sendToRef}></input>
                  </div>
                  <button className="button is-danger" id="handlePath" type="submit">Send</button>
                </form>
              </div>
              <div>
                {sendFundsResponse}
              </div>
            </section>
          </>
        }
      </div>
    </div >
  );
}

export default App;
