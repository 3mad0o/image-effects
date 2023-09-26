const path = require('path');
const HtmlWebPackPlugin=require('html-webpack-plugin');
const WasmPackPlugin =require('@wasm-tool/wasm-pack-plugin')
module.exports = {
  mode: 'development',
  entry: './public/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  plugins:[
    new HtmlWebPackPlugin({
        template:'./public/index.html'
    }),
    new WasmPackPlugin({
        crateDirectory:path.resolve(__dirname,'.')
    })
  ],
  experiments: {
    asyncWebAssembly: true,
  }
};