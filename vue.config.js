const { defineConfig } = require('@vue/cli-service');
const path = require('path');
const webpack = require('webpack');
module.exports = defineConfig({
  // ...其他配置

  chainWebpack: (config) => {
    config.resolve.alias
      .set('@', path.resolve(__dirname, 'src'))
      .set('@root', path.resolve(__dirname));
        config.module
          .rule('md')
          .test(/\.md$/)
          .use('gray-matter-loader')
          .loader('gray-matter-loader')
          .end();
  },

  configureWebpack: {
    plugins: [
      // Work around for Buffer is undefined:
      // https://github.com/webpack/changelog-v5/issues/10
      new webpack.ProvidePlugin({
          Buffer: ['buffer', 'Buffer'],
      }),
      new webpack.ProvidePlugin({
          process: 'process/browser',
      }),
  ],
    module: {
    },
  },

  transpileDependencies: true,
  lintOnSave: false,
});
