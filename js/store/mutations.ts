import { MutationTree } from 'vuex';

import MutationTypes from './mutation-types';
import { State } from './state';
import { Player, Stats, Players } from '../objects/player';

export type Mutations<S = State> = {
  [MutationTypes.DELETE_PLAYERS](state: S): void;
  [MutationTypes.CLEAR_EDIT_PLAYER](state: S): void;
  [MutationTypes.ADD_PLAYER](state: S, player: Player): void;
  [MutationTypes.IMPORT_PLAYERS](state: S, players: Players): void;
  [MutationTypes.UPDATE_STATS](state: S, udpate: { uuid: string; stats: Stats }): void;
  [MutationTypes.EDIT_PLAYER](state: S, playerId: string): void;
  [MutationTypes.DELETE_PLAYER](state: S, playerId: string): void;
};

export const mutations: MutationTree<State> & Mutations = {
  [MutationTypes.CLEAR_EDIT_PLAYER](state) {
    state.editPlayer = '';
  },
  [MutationTypes.ADD_PLAYER](state, player) {
    state.players[player.identity.uuid] = player;
  },
  [MutationTypes.IMPORT_PLAYERS](state, players) {
    state.players = { ...players, ...state.players };
  },
  [MutationTypes.UPDATE_STATS](state, { uuid, stats }) {
    state.players[uuid].stats = stats;
  },
  [MutationTypes.EDIT_PLAYER](state, playerId) {
    state.editPlayer = playerId;
  },
  [MutationTypes.DELETE_PLAYER](state, playerId) {
    delete state.players[playerId];
  },
  [MutationTypes.DELETE_PLAYERS](state) {
    state.players = {};
  },
};
