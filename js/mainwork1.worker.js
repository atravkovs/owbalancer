import * as Comlink from 'comlink';

const wasm = import('@/../pkg/index.js');

export default class WasmWorker {
  constructor() {
    this.loadWasm().then(() => {
      console.log("Wasm scripts loaded");
    });
  }

  async loadWasm() {
    this.ok = "ok";
    this.lib = await wasm;
  }

  test() {
    return new Promise((resolve) => {
      resolve(this.ok);
    });
  }

  fullBalance(data) {
    const { players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr, disableType, dispersionMinimizer, triesCount } = JSON.parse(data);

    return new Promise((resolve) => {
      const r = this.lib.balance(players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr, disableType, dispersionMinimizer, triesCount);
      resolve(r);
    });
  }

  halfBalance(data) {
    const { players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr } = JSON.parse(data);
    return new Promise((resolve) => {
      resolve(this.lib.balance_half(players, range, lowRankLimiter, disallowSecondaryRoles, adjustSr));
    });
  }

  finalBalance(data) {
    const { players, range, lowRankLimiter, disallowSecondaryRoles, reserveCopy, teamsCopy, adjustSr } = JSON.parse(data);
    return new Promise((resolve) => {
      resolve(this.lib.balance_final(players, range, lowRankLimiter, disallowSecondaryRoles, reserveCopy, teamsCopy, adjustSr));
    });
  }
}

Comlink.expose(WasmWorker);

