import { MutationTree } from 'vuex';
import { PLAYERS_IN_TEAM } from '@/constants';
import PObj, { Player, Stats, Players, ClassType } from '@/objects/player';
import TObj, { Teams, Team } from '@/objects/team';

import MutationTypes from './mutation-types';
import { State } from './state';

export type Mutations<S = State> = {
  [MutationTypes.CLEAR_TEAMS](state: S): void;
  [MutationTypes.TOGGLE_BALANCE](state: S): void;
  [MutationTypes.CLEAR_SQUIRES](state: S): void;
  [MutationTypes.CLEAR_CAPTAINS](state: S): void;
  [MutationTypes.DELETE_PLAYERS](state: S): void;
  [MutationTypes.CLEAR_ALL_EXTRA](state: S): void;
  [MutationTypes.CLEAR_EDIT_PLAYER](state: S): void;
  [MutationTypes.ADD_TEAM](state: S, team: Team): void;
  [MutationTypes.ADD_TEAMS](state: S, teams: Teams): void;
  [MutationTypes.ADD_RESERVE](state: S, uuid: string): void;
  [MutationTypes.ADD_PLAYER](state: S, player: Player): void;
  [MutationTypes.REMOVE_TEAM](state: S, teamUuid: string): void;
  [MutationTypes.EDIT_PLAYER](state: S, playerId: string): void;
  [MutationTypes.ASSIGN_SQUIRES](state: S, maxSR: number): void;
  [MutationTypes.ADD_PLAYERS](state: S, players: Players): void;
  [MutationTypes.ASSIGN_CAPTAINS](state: S, minSR: number): void;
  [MutationTypes.DELETE_PLAYER](state: S, playerId: string): void;
  [MutationTypes.IMPORT_PLAYERS](state: S, players: Players): void;
  [MutationTypes.IMPORT_PLAYERS_OLD](state: S, data: string): void;
  [MutationTypes.RESERVE_PLAYERS](state: S, players: string[]): void;
  [MutationTypes.REMOVE_FROM_RESERVE](state: S, playerId: string): void;
  [MutationTypes.UPDATE_STATS](state: S, udpate: { uuid: string; stats: Stats }): void;
  [MutationTypes.REMOVE_FROM_TEAM](state: S, data: { teamUuid: string; playerId: string }): void;
  [MutationTypes.UPDATE_TEAM_NAME](state: S, data: { teamUuid: string; teamName: string }): void;
  [MutationTypes.EDIT_RANK](state: S, data: { uuid: string; rank: number; role: 'dps' | 'support' | 'tank' }): void;
  [MutationTypes.ADD_TEAMPLAYER](state: S, data: { teamUuid: string; playerName: string; playerId: string; role: ClassType; primary: boolean; secondary: boolean; roleName: 'dps' | 'support' | 'tank' }): void;
};

export const mutations: MutationTree<State> & Mutations = {
  [MutationTypes.EDIT_RANK](state, { uuid, rank, role }) {
    state.players[uuid].stats.classes[role].rank = rank;
  },
  [MutationTypes.UPDATE_TEAM_NAME](state, { teamUuid, teamName }) {
    const index = state.teams.findIndex(mTeam => mTeam.uuid === teamUuid);

    if (index >= 0) {
      state.teams[index].name = teamName;
    }
  },
  [MutationTypes.CLEAR_EDIT_PLAYER](state) {
    state.editPlayer = '';
  },
  [MutationTypes.TOGGLE_BALANCE](state) {
    state.isBalance = !state.isBalance;
  },
  [MutationTypes.ADD_PLAYER](state, player) {
    state.players[player.identity.uuid] = player;
  },
  [MutationTypes.ADD_RESERVE](state, uuid) {
    state.reservedPlayers.push(uuid);
  },
  [MutationTypes.ADD_PLAYERS](state, players) {
    state.players = { ...state.players, ...players };
  },
  [MutationTypes.ADD_TEAM](state, team) {
    state.teams.push(team);
  },
  [MutationTypes.ADD_TEAMS](state, teams) {
    state.teams = [...teams];
  },
  [MutationTypes.REMOVE_TEAM](state, teamUuid: string) {
    const teamIndex = state.teams.findIndex(tm => tm.uuid === teamUuid);

    if (teamIndex < 0) {
      return;
    }

    if (state.teams.length - 1 > 0) {
      const players = state.teams[teamIndex].members.map(member => member.uuid);
      state.reservedPlayers.push(...players);
    } else {
      state.reservedPlayers = [];
    }

    state.teams.splice(teamIndex, 1);
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
  [MutationTypes.REMOVE_FROM_RESERVE](state, playerId) {
    const index = state.reservedPlayers.indexOf(playerId);
    if (index >= 0) {
      state.reservedPlayers.splice(index, 1);
    }
  },
  [MutationTypes.REMOVE_FROM_TEAM](state, { teamUuid, playerId }) {
    const teamF = state.teams.findIndex(team => team.uuid === teamUuid);

    if (teamF !== -1) {
      const index = state.teams[teamF].members.findIndex(member => member.uuid === playerId);
      state.teams[teamF].members.splice(index, 1);
      const { avgSr, totalSr } = TObj.calcStats(state.teams[teamF]);
      state.teams[teamF].avgSr = avgSr;
      state.teams[teamF].totalSr = totalSr;
    }
  },
  [MutationTypes.ADD_TEAMPLAYER](state, { playerName, teamUuid, playerId, role, roleName, primary, secondary }) {
    const teamF = state.teams.findIndex(team => team.uuid === teamUuid);

    if (teamF !== -1) {
      state.teams[teamF].members.push({
        uuid: playerId,
        name: playerName,
        rank: role.rank,
        role: roleName,
        primary, secondary,
      });
      const { avgSr, totalSr } = TObj.calcStats(state.teams[teamF]);
      state.teams[teamF].avgSr = avgSr;
      state.teams[teamF].totalSr = totalSr;
    }
  },
  [MutationTypes.ASSIGN_CAPTAINS](state, minSR) {
    const players = Object.entries(state.players);
    const captainsCount = Math.floor(players.length / PLAYERS_IN_TEAM);
    const eligible = players.filter(([, player]) => PObj.getTopRank(player) >= minSR)
      .sort(([, player], [, player2]) => PObj.getTopRank(player2) - PObj.getTopRank(player));

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
    const eligible = players.filter(([, player]) => PObj.getTopRank(player) <= maxSR)
      .sort(([, player], [, player2]) => PObj.getTopRank(player) - PObj.getTopRank(player2));

    let i = 0;
    while (i < squiresCount && i < eligible.length) {
      const [uuid] = eligible[i];
      state.players[uuid].identity.isSquire = true;
      i += 1;
    }
  },
  [MutationTypes.CLEAR_TEAMS](state) {
    state.teams = [];
    state.reservedPlayers = [];
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
  },
  [MutationTypes.RESERVE_PLAYERS](state, players) {
    state.reservedPlayers = players;
  },
  [MutationTypes.IMPORT_PLAYERS_OLD](state, data) {
    const parsedData = JSON.parse(data);
    const { players } = parsedData;
    const newPlayers: Players = {};

    // eslint-disable-next-line
    players.forEach((player: any) => {
      const basePlayer = PObj.createDefaultPlayer(player.display_name);

      if (player.sr_by_class.dps) {
        basePlayer.stats.classes.dps.rank = player.sr_by_class.dps;
      }
      if (player.sr_by_class.tank) {
        basePlayer.stats.classes.tank.rank = player.sr_by_class.tank;
      }
      if (player.sr_by_class.support) {
        basePlayer.stats.classes.support.rank = player.sr_by_class.support;
      }

      let i = 0;
      player.classes.forEach((rank: string) => {
        const ct = PObj.getRole(basePlayer.stats.classes, rank);
        ct.priority = i;
        ct.isActive = true;
        i += 1;
      });

      newPlayers[basePlayer.identity.uuid] = basePlayer;
    });

    state.players = { ...state.players, ...newPlayers };
  },
};
