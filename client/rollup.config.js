import { getBabelOutputPlugin } from '@rollup/plugin-babel';
import scss from "rollup-plugin-scss";
import postcss from "postcss";
import autoprefixer from "autoprefixer";
import path from "node:path";
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default [
    {
        // ES6
        input: './client/src/static/index.js',
        plugins: [
            scss({
                output: "./client/public/build/style.css",
                failOnError: true,
                outputStyle: 'compressed'
            })
        ],
        output: {
            file: './client/public/build/bundle.js',
            format: 'es'
        },
    },
    {
        // ES5
        input: './client/src/static/index.js',
        plugins: [
            getBabelOutputPlugin({
                presets: ['@babel/preset-env']
            }),
            scss({
                output: "./client/public/build/style.css",
                failOnError: true,
                outputStyle: 'compressed'
            }),
        ],
        output: {
            file: './client/public/build/bundle-es5.js',
            format: 'cjs'
        }
    }
];
