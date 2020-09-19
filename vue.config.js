const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

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
    ],
    devServer: {
      clientLogLevel: 'info',
      watchOptions: {
        poll: true,
      },
    },
  },
};
