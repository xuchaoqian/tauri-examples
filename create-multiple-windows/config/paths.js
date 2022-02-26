const path = require("path");

const root = path.resolve(__dirname, "..");
const build = path.resolve(root, "build");
const public = path.resolve(root, "public");
const src = path.resolve(root, "src");
const localModules = path.resolve(root, "local-modules");
const webRoot = path.resolve(localModules, "landbridge-web");
const webSrc = path.resolve(webRoot, "src");

module.exports = {
  root,
  build,
  public,
  src,
  localModules,
  webRoot,
  webSrc,
};
