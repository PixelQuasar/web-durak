import { getBabelOutputPlugin } from '@rollup/plugin-babel';

export default [
    {
        // ES6
        input: './client/src/main.js',
        output: {
            file: './client/build/bundle.js',
            format: 'es'
        },
    },
    {
        // ES5
        input: './client/src/main.js',
        plugins: [
            getBabelOutputPlugin({
                presets: ['@babel/preset-env']
            })
        ],
        output: {
            file: './client/build/bundle-es5.js',
            format: 'cjs'
        }
    }
];
