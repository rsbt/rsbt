const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/index.tsx",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      { test: /.html$/, use: "raw-loader" },
      { test: /\.json$/, use: "json-loader" },
      {
        test: /\.(s*)css$/,
        use: [
          "style-loader",
          {
            loader: "css-loader",
            options: {
              url: (url, resourcePath) => {
                if (url.includes("/res/")) {
                  return false;
                }

                return true;
              },
            },
          },
        ],
      },
      {
        test: /\.woff(\?.+)?$/,
        use: "url-loader?limit=10000&mimetype=application/font-woff",
      },
      {
        test: /\.woff2(\?.+)?$/,
        use: "url-loader?limit=10000&mimetype=application/font-woff",
      },
      { test: /\.ttf(\?.+)?$/, use: "file-loader" },
      { test: /\.eot(\?.+)?$/, use: "file-loader" },
      { test: /\.svg(\?.+)?$/, use: "file-loader" },
      { test: /\.png$/, use: "url-loader?mimetype=image/png" },
      { test: /\.gif$/, use: "url-loader?mimetype=image/gif" },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "Output Management",
      template: "./src/index.html",
    }),
  ],
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
};
