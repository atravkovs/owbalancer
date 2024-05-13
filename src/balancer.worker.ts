import * as Comlink from 'comlink';
import init, { balance, balance_half, balance_final } from '@rust/owbalancer';
import wasmFileUrl from '@rust/owbalancer/owbalancer_bg.wasm?url';

export default class WasmWorker {
  private ok: string = '';

  constructor() {
    this.loadWasm().then(() => {
      console.log('Wasm scripts loaded');
    });
  }

  async loadWasm() {
    this.ok = 'ok';
    await init(wasmFileUrl);
  }

  test() {
    return new Promise((resolve) => {
      resolve(this.ok);
    });
  }

  fullBalance(data) {
    const {
      players,
      range,
      lowRankLimiter,
      disallowSecondaryRoles,
      adjustSr,
      disableType,
      dispersionMinimizer,
      triesCount,
    } = JSON.parse(data);

    return new Promise((resolve) => {
      const r = balance(
        players,
        range,
        lowRankLimiter,
        disallowSecondaryRoles,
        adjustSr,
        disableType,
        dispersionMinimizer,
        triesCount
      );

      resolve(r);
    });
  }

  halfBalance(data) {
    const { players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr } = JSON.parse(data);
    return new Promise((resolve) => {
      resolve(balance_half(players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr));
    });
  }

  finalBalance(data) {
    const {
      players,
      range,
      lowRankLimiter,
      disallowSecondaryRoles,
      reserveCopy,
      teamsCopy,
      adjustSr,
    } = JSON.parse(data);
    return new Promise((resolve) => {
      resolve(
        balance_final(
          players,
          range,
          lowRankLimiter,
          disallowSecondaryRoles,
          reserveCopy,
          teamsCopy,
          adjustSr
        )
      );
    });
  }
}

Comlink.expose(WasmWorker);
