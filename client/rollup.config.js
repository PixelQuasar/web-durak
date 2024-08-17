import { getBabelOutputPlugin } from '@rollup/plugin-babel';
import scss from "rollup-plugin-scss";
import terser from '@rollup/plugin-terser';
import dotenv from "rollup-plugin-dotenv";

export default [
    {
        // ES6
        input: './client/src/static/index.js',
        plugins: [
            scss({
                output: "./client/build/style.css",
                failOnError: true,
                outputStyle: 'compressed'
            }),
            //terser(),
            dotenv()
        ],
        output: {
            file: './client/build/bundle.js',
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
                output: "./client/build/style.css",
                failOnError: true,
                outputStyle: 'compressed'
            }),
            terser(),
            dotenv()
        ],
        output: {
            file: './client/build/bundle-es5.js',
            format: 'cjs'
        }
    }
];
