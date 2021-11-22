import React from "react";
import { window as tauriWindow } from "@tauri-apps/api";
import logo from "./logo.svg";
import tauriCircles from "./tauri.svg";
import tauriWord from "./wordmark.svg";
import "./App.css";

window.addEventListener("DOMContentLoaded", (event: any) => {
  document.addEventListener(
    "click",
    (e: any) => {
      if (e.target.matches("a")) {
        new tauriWindow.WebviewWindow("theUniqueLabel", {
          url: "https://www.apple.com",
        });
        e.preventDefault();
        e.stopPropagation();
        return false;
      }
    },
    { capture: true }
  );
});

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <div className="inline-logo">
          <img src={tauriCircles} className="App-logo rotate" alt="logo" />
          <img src={tauriWord} className="App-logo smaller" alt="logo" />
        </div>
        <a
          className="App-link"
          href="https://tauri.studio"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Tauri
        </a>
        <img src={logo} className="App-logo rotate" alt="logo" />
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
      </header>
    </div>
  );
}

export default App;
