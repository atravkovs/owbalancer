// ! Temporary measure, while no meaningfull Actions do exist
/* eslint-disable */

import { ActionContext, ActionTree } from 'vuex';

import ActionTypes from './action-types';
import { Mutations } from './mutations';
import { State } from './state';

type AugmentedActionContext = {
  commit<K extends keyof Mutations>(
    key: K,
    payload: Parameters<Mutations[K]>[1]
  ): ReturnType<Mutations[K]>;
} & Omit<ActionContext<State, State>, 'commit'>;

export interface Actions {
  [ActionTypes.ADD_PLAYER]({ commit }: AugmentedActionContext, playerId: string): void;
}

export const actions: ActionTree<State, State> & Actions = {
  [ActionTypes.ADD_PLAYER]({ commit }, playerId) {
    // commit(MutationTypes.ADD_PLAYER, playerId);
  },
};
