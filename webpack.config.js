const webpack = require('webpack');
const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const config = {
  dist: path.resolve(__dirname, 'dist'),
  src: path.resolve(__dirname, 'src'),
};

module.exports = {
  mode: 'development', 
  entry: [
    './src/app/index.ts'
  ],
  devServer: {
    contentBase: config.dist,
    compress: false,
    port: 9000,
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
      "Access-Control-Allow-Headers": "X-Requested-With, content-type, Authorization"
    },
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader'
      }
    ]
  },
  resolve: {
    extensions: [
      '.ts', '.js',
    ]
  },
  plugins: [
    new CleanWebpackPlugin(['dist'], {
      root: path.resolve(__dirname),
    }),
    new HtmlWebpackPlugin({
      inject: true,
      template: `./src/index.html`,
    }),
  ],

  output: {
    filename: '[name].bundle.js',
    path:  config.dist,
  },
};
