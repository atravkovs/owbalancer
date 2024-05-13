// eslint-disable-next-line import/prefer-default-export, @typescript-eslint/camelcase
export function wasm_log(message) {
    self.postMessage({ type: 'pop', message }); /* eslint-disable-line no-restricted-globals */
}
