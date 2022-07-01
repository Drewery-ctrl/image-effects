const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
   entry: './public/main.js',
   output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'bundle.js'
   },
   plugins: [
      new HTMLWebpackPlugin({
         template: './public/index.html'
      }),
      new WasmPackPlugin({
         crateDirectory: path.resolve(__dirname, '.'), // searches for cargo.toml file
      })
   ],
   experiments: {
      asyncWebAssembly: true,
      topLevelAwait: true
   }
}
