import { MutationTree } from 'vuex';
import { PLAYERS_IN_TEAM } from '@/constants';

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
  [MutationTypes.ASSIGN_CAPTAINS](state: S, minSR: number): void;
  [MutationTypes.ASSIGN_SQUIRES](state: S, maxSR: number): void;
  [MutationTypes.CLEAR_CAPTAINS](state: S): void;
  [MutationTypes.CLEAR_SQUIRES](state: S): void;
  [MutationTypes.CLEAR_ALL_EXTRA](state: S): void;
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
  [MutationTypes.ASSIGN_CAPTAINS](state, minSR) {
    const players = Object.entries(state.players);
    const captainsCount = Math.floor(players.length / PLAYERS_IN_TEAM);
    const eligible = players.filter(([, player]) => player.stats.rank >= minSR)
      .sort(([, player], [, player2]) => player2.stats.rank - player.stats.rank);

    let i = 0;
    while (i < captainsCount && i < eligible.length) {
      const [uuid] = eligible[i];
      state.players[uuid].identity.isCaptain = true;
      i += 1;
    }
  },
  [MutationTypes.ASSIGN_SQUIRES](state, maxSR) {
    const players = Object.entries(state.players);
    const squiresCount = Math.floor(players.length / PLAYERS_IN_TEAM);
    const eligible = players.filter(([, player]) => player.stats.rank <= maxSR)
      .sort(([, player], [, player2]) => player.stats.rank - player2.stats.rank);

    let i = 0;
    while (i < squiresCount && i < eligible.length) {
      const [uuid] = eligible[i];
      state.players[uuid].identity.isSquire = true;
      i += 1;
    }
  },
  [MutationTypes.CLEAR_CAPTAINS](state) {
    Object.entries(state.players).filter(([, player]) => player.identity.isCaptain === true).forEach(([uuid]) => {
      state.players[uuid].identity.isCaptain = false;
    });
  },
  [MutationTypes.CLEAR_SQUIRES](state) {
    Object.entries(state.players).filter(([, player]) => player.identity.isSquire === true).forEach(([uuid]) => {
      state.players[uuid].identity.isSquire = false;
    });
  },
  [MutationTypes.CLEAR_ALL_EXTRA](state) {
    Object.entries(state.players).filter(([, player]) => player.identity.isSquire === true || player.identity.isCaptain === true).forEach(([uuid]) => {
      state.players[uuid].identity.isCaptain = false;
      state.players[uuid].identity.isSquire = false;
    });
  }
};
