import { createApp } from 'vue';
import App from './App.vue';
import { store } from './store';

const wasm = import("../pkg/index.js");

wasm.then(lib => {
  console.log('Hi!');
  const text = lib.greet('Bob');
  console.log('Received: ', text);
}).catch(console.error);

createApp(App)
  .use(store)
  .mount('#app');
