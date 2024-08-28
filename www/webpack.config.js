const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = (env, options) => ({
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({ 
      patterns: [
        { from: 'index.html', to: 'dist/index.html' }
      ]
    })
  ],
  experiments: {
    asyncWebAssembly: true
    // syncWebAssembly: true
  }
});
