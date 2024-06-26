import * as Comlink from 'comlink';

// const MyWorker = new Worker('./mainwork1.worker.js', { name: 'cutie', type: 'module' });
import BalancerWorker from './balancer.worker?worker';

const instance = new BalancerWorker();

// eslint-disable-next-line
const Wrap: any = Comlink.wrap(instance);

instance.addEventListener('message', (m) => {
  if (m.data?.type === 'pop') {
    document.dispatchEvent(
      new CustomEvent('wasm-update', { detail: { message: () => m.data.message } })
    );
  }
});

const getIt = async () => {
  const WasmWorker = await new Wrap();
  return WasmWorker;
};

export default getIt();
