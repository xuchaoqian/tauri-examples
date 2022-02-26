const webpack = require("webpack");
const merge = require("webpack-merge");
const base = require("./webpack.base.js");
const paths = require("./paths");
const { getEnvVars } = require("./env-vars");

module.exports = (env) =>
  merge.smart(base(env), {
    mode: "development",
    plugins: [new webpack.DefinePlugin(getEnvVars("development"))],
    devServer: {
      historyApiFallback: true,
      compress: true,
      clientLogLevel: "none",
      contentBase: paths.public,
      watchContentBase: true,
      hot: true,
      publicPath: "/",
      overlay: false,
      host: "0.0.0.0",
      port: "3000"
    },
    devtool: "source-map",
  });
