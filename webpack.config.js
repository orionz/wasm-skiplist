const path = require("path");
const microtime = require("microtime");

module.exports = {
  entry: "./index.js",
  target: "node",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    library: 'SkipList',
    libraryTarget: 'umd',
  },
  mode: "development"
};
