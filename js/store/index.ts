import { createStore, Store as VuexStore, CommitOptions, DispatchOptions } from 'vuex';

import { State, state } from './state';
import { Getters, getters } from './getters';
import { actions, Actions } from './actions';
import { mutations, Mutations } from './mutations';
import plugins from './plugins';

export const store = createStore({
  state,
  getters,
  actions,
  mutations,
  plugins,
});

export function useStore() {
  return store as Store;
}

export type Store = Omit<VuexStore<State>, 'getters' | 'commit' | 'dispatch'> & {
  commit<K extends keyof Mutations, P extends Parameters<Mutations[K]>[1]>(
    key: K,
    payload: P,
    options?: CommitOptions
  ): ReturnType<Mutations[K]>;
} & {
  dispatch<K extends keyof Actions>(
    key: K,
    payload: Parameters<Actions[K]>[1],
    options?: DispatchOptions
  ): ReturnType<Actions[K]>;
} & {
  getters: {
    [K in keyof Getters]: ReturnType<Getters[K]>;
  };
};
