const webpack = require("webpack");
const merge = require("webpack-merge");
const TerserPlugin = require("terser-webpack-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const base = require("./webpack.base.js");
const { getEnvVars } = require("./env-vars");

module.exports = (env) =>
  merge.smart(base(env), {
    mode: "production",
    optimization: {
      minimize: true,
      minimizer: [
        new TerserPlugin({
          terserOptions: {
            parse: {
              // We want terser to parse ecma 8 code. However, we don't want it
              // to apply any minification steps that turns valid ecma 5 code
              // into invalid ecma 5 code. This is why the 'compress' and 'output'
              // sections only apply transformations that are ecma 5 safe
              // https://github.com/facebook/create-react-app/pull/4234
              ecma: 8,
            },
            // compress: {
            // 	ecma: 5,
            // 	// Disabled because of an issue with Uglify breaking seemingly valid code:
            // 	// https://github.com/facebook/create-react-app/issues/2376
            // 	// Pending further investigation:
            // 	// https://github.com/mishoo/UglifyJS2/issues/2011
            // 	comparisons: false,
            // 	inline: 2,
            // 	warnings: false,
            // 	drop_console: true,
            // 	drop_debugger: true,
            // 	pure_funcs: ['console.log'],
            // },
            mangle: {
              safari10: true,
            },
            keep_classnames: true,
            keep_fnames: true,
            output: {
              ecma: 5,
              comments: false,
              // Turned on because emoji and regex is not minified properly using default
              // https://github.com/facebook/create-react-app/issues/2488
              ascii_only: true,
            },
          },
          parallel: true,
        }),
        new CssMinimizerPlugin(),
      ],
    },
    plugins: [new webpack.DefinePlugin(getEnvVars("production"))],
  });
