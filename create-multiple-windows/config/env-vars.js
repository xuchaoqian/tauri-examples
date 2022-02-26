const process = require("process");

function buildOS() {
  if (process.platform === "darwin") {
    return "macOS";
  } else if (process.platform === "linux") {
    return "Linux";
  } else if (process.platform.startsWith("win")) {
    return "Windows";
  } else {
    return "UnknowOS";
  }
}

function getEnvVars(mod) {
  if (mod === "development") {
    return {
      "process.env.NODE_ENV": JSON.stringify("development"),
      "process.env.OS": JSON.stringify(buildOS()),
      "process.env.TARGET": JSON.stringify("desktop"),
    };
  } else {
    return {
      "process.env.NODE_ENV": JSON.stringify("production"),
      "process.env.OS": JSON.stringify(buildOS()),
      "process.env.TARGET": JSON.stringify("desktop"),
    };
  }
}

module.exports = { getEnvVars };
