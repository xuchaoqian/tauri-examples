const path = require("path");
const process = require("process");
const webpack = require("webpack");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const paths = require("./paths");

function buildTargets() {
  if (process.platform === "darwin") {
    return { safari: "14" };
  } else if (process.platform.startsWith("win")) {
    return { edge: "86" };
  } else {
    return "> 1% or last 2 versions";
  }
}

module.exports = (env) => {
  return {
    context: paths.root,
    entry: {
      index: path.resolve(paths.src, "main-index.tsx"),
      splash: path.resolve(paths.src, "splash-index.tsx")
    },
    output: {
      path: path.resolve(paths.build),
      filename: "js/[name].[contenthash:8].js",
      chunkFilename: "js/[name].[contenthash:8].chunk.js",
      pathinfo: true,
      publicPath: "/",
    },
    module: {
      rules: [
        // JS or JSX support
        {
          test: /\.(?:js|jsx|ts|tsx)$/,
          exclude: [/node_modules/, /chartLibrary/],
          use: [
            {
              loader: "babel-loader",
              options: {
                cacheDirectory: true,
                presets: [
                  [
                    "@babel/preset-env",
                    {
                      targets: buildTargets(),
                      useBuiltIns: "usage",
                      corejs: { version: "3.10", proposals: true },
                    },
                  ],
                  "@babel/preset-typescript",
                  "@babel/preset-react",
                ],
                plugins: [
                  [
                    "@babel/plugin-transform-async-to-generator",
                  ],
                  [
                    "@babel/plugin-proposal-decorators",
                    {
                      legacy: true,
                    },
                  ],
                  [
                    "@babel/plugin-proposal-class-properties",
                    {
                      loose: true,
                    },
                  ],
                  ["@babel/plugin-proposal-private-methods", { loose: true }],
                  [
                    "@babel/plugin-proposal-private-property-in-object",
                    { loose: true },
                  ],
                  "@babel/plugin-proposal-object-rest-spread",
                  "@babel/plugin-syntax-dynamic-import",
                  [
                    "import",
                    {
                      libraryName: "antd",
                      libraryDirectory: "es",
                      style: true,
                    },
                  ],
                ],
              },
            },
          ],
        },

        // CSS/SASS/LESS support
        {
          test: /\.css$/,
          use: ["style-loader", "css-loader"],
        },
        {
          test: /\.less$/,
          use: [
            // MiniCssExtractPlugin.loader,
            "style-loader",
            "css-loader",
            {
              loader: "less-loader", // Compiles Less to CSS
              options: {
                lessOptions: {
                  javascriptEnabled: true,
                  modifyVars: {
                    "primary-color": "#ea8c00",
                    "menu-dark-item-selected": "#ffffff",
                    "menu-dark-bg": "#191919",
                    "menu-dark-submenu-bg": "#191919",
                    "menu-dark-inline-submenu-bg": "#191919",
                  },
                },
              },
            },
          ],
        },
        {
          test: /\.(?:sass|scss)$/,
          exclude: [path.resolve(paths.webSrc, "pages", "vip")],
          use: [
            // MiniCssExtractPlugin.loader,
            "style-loader",
            {
              loader: "css-loader", // Translates CSS into CommonJS
              options: {
                sourceMap: true,
                modules: {
                  localIdentName: "[name]__[local]__[hash:base64:5]",
                  exportLocalsConvention: "camelCase",
                },
              },
            },
            "sass-loader", // Compiles Sass to CSS
          ],
        },

        // Image support
        {
          test: /\.(?:bmp|ico|gif|png|jpg|jpeg|webp|svg)$/,
          use: [
            {
              loader: "url-loader",
              options: {
                limit: 10000,
                name: "web/images/[name].[ext]",
                esModule: false,
              },
            },
            {
              loader: "image-webpack-loader",
              options: {
                bypassOnDebug: true,
              },
            },
          ],
        },

        // Audio support
        {
          test: /\.(?:mp3|wav)$/,
          use: [
            {
              loader: "url-loader",
              options: {
                name: "sounds/[name].[ext]",
              },
            },
          ],
        },

        // Fonts support
        {
          test: /\.(?:woff|woff2|eot|otf)$/,
          use: [
            {
              loader: "url-loader",
              options: {
                name: "fonts/[name].[ext]",
                esModule: false,
              },
            },
          ],
        },
        {
          test: /\.ttf$/,
          use: ["file-loader"],
        },
      ],
    },
    plugins: [
      new webpack.ProgressPlugin(),
      new CleanWebpackPlugin({
        root: paths.root,
      }),
      new HtmlWebpackPlugin({
        inject: true,
        filename: "index.html",
        template: path.resolve(paths.public, "index.html"),
        chunks: ["index"],
      }),
      new HtmlWebpackPlugin({
        inject: true,
        filename: "splash.html",
        template: path.resolve(paths.public, "splash.html"),
        chunks: ["splash"],
      }),
    ],
    node: {
      global: false,
      __filename: true,
      __dirname: true,
    },
    resolve: {
      extensions: [".js", ".jsx", ".ts", ".tsx", ".ttf"],
      fallback: {
        path: require.resolve("path-browserify"),
        fs: require.resolve("fs"),
      },
    },
    externals: {
      fs: "fs",
    },
    target: "web",
  };
};
