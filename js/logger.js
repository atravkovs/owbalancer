export function wasm_log(message) {
    document.dispatchEvent(new CustomEvent('wasm-update', { detail: { message: () => message } }));
}
