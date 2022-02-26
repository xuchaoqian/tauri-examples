import * as React from "react";
import { window as tauriWindow } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/tauri";
import Logo from "./logo.svg";
import Loading from "./loading.gif";
import styles from "./Splash.scss";

async function createMultiWindows(): Promise<void> {
  return invoke("create_multi_windows");
}

async function showMultiWindows(): Promise<void> {
  return invoke("show_multi_windows");
}

window.addEventListener("DOMContentLoaded", async (event: any) => {
  try {
    await createMultiWindows();
    await new Promise((resolve) => setTimeout(resolve, 1000));
    // await tauriWindow.getCurrent().hide();
    await showMultiWindows();
  } catch (error) {
    console.error(`Failed to apply layout: ${error}`);
  } finally {
    // await tauriWindow.getCurrent().close();
  }
});

export class Splash extends React.Component {
  componentDidMount() { }
  render() {
    return (
      <div className={styles["loading-wrap"]}>
        <img src={Logo} alt="logo" className={styles["logo-img"]} />
        <img src={Loading} alt="loading" className={styles["loading-img"]} />
      </div>
    );
  }
}

export default Splash;
