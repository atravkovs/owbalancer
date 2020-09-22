const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const { DefinePlugin } = require('webpack');

module.exports = {
  configureWebpack: {
    entry: './js/main.ts',
    resolve: {
      alias: {
        '@': path.join(__dirname, './js'),
      },
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: __dirname,
      }),
      new DefinePlugin({
        '__VUE_OPTIONS_API__': false,
        '__VUE_PROD_DEVTOOLS__': false,
      }),
    ],
    devServer: {
      clientLogLevel: 'info',
      watchOptions: {
        poll: true,
      },
    },
  },
};
