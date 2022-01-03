import { MutationTree } from 'vuex';

import cloneDeep from 'lodash/cloneDeep';

import { PLAYERS_IN_TEAM } from '@/constants';
import PObj, { Player, Stats, Players, ClassType, LobbyType } from '@/objects/player';
import TObj, { Teams, Team } from '@/objects/team';
import { BalancerOptions, Results } from '@/objects/balance';
import { ArchiveEntry } from '@/objects/archive';

import MutationTypes from './mutation-types';
import { State } from './state';

export type Mutations<S = State> = {
  [MutationTypes.CLEAR_TEAMS](state: S, _: undefined): void;
  [MutationTypes.EMPTY_TEAMS](state: S, _: undefined): void;
  [MutationTypes.EMPTY_NO_RANK](state: S, _: undefined): void;
  [MutationTypes.TOGGLE_BACKUP](state: S, _: undefined): void;
  [MutationTypes.TOGGLE_ARCHIVE](state: S, _: undefined): void;
  [MutationTypes.TOGGLE_BALANCE](state: S, _: undefined): void;
  [MutationTypes.SAVE_TO_ARCHIVE](state: S, _: undefined): void;
  [MutationTypes.TOGGLE_SELECTION](state: S, _: undefined): void;
  [MutationTypes.TOGGLE_BALANCER_SR](state: S, _: undefined): void;
  [MutationTypes.CLEAR_SQUIRES](state: S, _: undefined): void;
  [MutationTypes.CLEAR_CAPTAINS](state: S, _: undefined): void;
  [MutationTypes.DELETE_PLAYERS](state: S, lobby?: LobbyType): void;
  [MutationTypes.CLEAR_ALL_EXTRA](state: S, _: undefined): void;
  [MutationTypes.CLEAR_EDIT_PLAYER](state: S, _: undefined): void;
  [MutationTypes.ADD_TEAM](state: S, team: Team): void;
  [MutationTypes.ADD_TEAMS](state: S, teams: Teams): void;
  [MutationTypes.ADD_RESERVE](state: S, uuid: string): void;
  [MutationTypes.ADD_PLAYER](state: S, data: { player: Player; lobby?: LobbyType }): void;
  [MutationTypes.SELECT_PLAYERS](state: S, data: { playerIds: string[]; lobby: LobbyType }): void;
  [MutationTypes.SELECT_ARCHIVE](state: S, id: number): void;
  [MutationTypes.REMOVE_TEAM](state: S, teamUuid: string): void;
  [MutationTypes.EDIT_PLAYER](state: S, data: { playerId: string; lobby?: LobbyType }): void;
  [MutationTypes.ASSIGN_SQUIRES](state: S, maxSR: number): void;
  [MutationTypes.ADD_PLAYERS](state: S, data: { players: Players; lobby?: LobbyType }): void;
  [MutationTypes.ASSIGN_CAPTAINS](state: S, minSR: number): void;
  [MutationTypes.REMOVE_FROM_ARCHIVE](state: S, id: number): void;
  [MutationTypes.DELETE_PLAYER](state: S, data: { playerId: string; lobby?: LobbyType }): void;
  [MutationTypes.SET_RESULTS](state: S, results: Results): void;
  [MutationTypes.IMPORT_PLAYERS](state: S, data: { players: Players; lobby?: LobbyType }): void;
  [MutationTypes.IMPORT_ARCHIVE](state: S, data: ArchiveEntry): void;
  [MutationTypes.IMPORT_PLAYERS_OLD](state: S, data: string): void;
  [MutationTypes.RESERVE_PLAYERS](state: S, players: string[]): void;
  [MutationTypes.REMOVE_FROM_RESERVE](state: S, playerId: string): void;
  [MutationTypes.SET_BALANCER_OPTIONS](state: S, options: BalancerOptions): void;
  [MutationTypes.UPDATE_STATS](state: S, update: { uuid: string; stats: Stats; lobby?: LobbyType }): void;
  [MutationTypes.UPDATE_ARCHIVE_NAME](state: S, update: { id: number; name: string }): void;
  [MutationTypes.REMOVE_FROM_TEAM](state: S, data: { teamUuid: string; playerId: string }): void;
  [MutationTypes.UPDATE_TEAM_NAME](state: S, data: { teamUuid: string; teamName: string }): void;
  [MutationTypes.EDIT_RANK](
    state: S,
    data: { uuid: string; rank: number; role: 'dps' | 'support' | 'tank' }
  ): void;
  [MutationTypes.EDIT_SPECIALIZATION](
    state: S,
    data: { uuid: string; value: boolean; specialization: 'primary' | 'secondary'; role: 'dps' | 'support' | 'tank' }
  ): void;
  [MutationTypes.ADD_TEAMPLAYER](
    state: S,
    data: {
      teamUuid: string;
      playerName: string;
      playerId: string;
      role: ClassType;
      primary: boolean;
      secondary: boolean;
      roleName: 'dps' | 'support' | 'tank';
    }
  ): void;
};

export const mutations: MutationTree<State> & Mutations = {
  [MutationTypes.SET_RESULTS](state, results) {
    state.balancerResults = results;
  },
  [MutationTypes.EDIT_RANK](state, { uuid, rank, role }) {
    if (state.players[uuid]) state.players[uuid].stats.classes[role].rank = rank;
    if (state.backup[uuid]) state.backup[uuid].stats.classes[role].rank = rank;
  },
  [MutationTypes.EDIT_SPECIALIZATION](state, { uuid, value, specialization, role }) {
    if (state.players[uuid]) state.players[uuid].stats.classes[role][specialization] = value;
    if (state.backup[uuid]) state.backup[uuid].stats.classes[role][specialization] = value;
  },
  [MutationTypes.UPDATE_TEAM_NAME](state, { teamUuid, teamName }) {
    const index = state.teams.findIndex(mTeam => mTeam.uuid === teamUuid);

    if (index >= 0) {
      state.teams[index].name = teamName;
    }
  },
  [MutationTypes.UPDATE_ARCHIVE_NAME](state, { id, name }) {
    state.archive[id].name = name;
  },
  [MutationTypes.EMPTY_NO_RANK](state) {
    if (state.editPlayer.playerId === '') return;

    if (state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.dps.rank === 0) {
      state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.dps.isActive = false;
    }
    if (state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.tank.rank === 0) {
      state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.tank.isActive = false;
    }
    if (state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.support.rank === 0) {
      state[state.editPlayer.lobby][state.editPlayer.playerId].stats.classes.support.isActive = false;
    }
  },
  [MutationTypes.CLEAR_EDIT_PLAYER](state) {
    /* Sync Backup Identity with Players */
    const { playerId, lobby } = state.editPlayer;
    const to = lobby === 'players' ? 'backup' : 'players';

    if (state[to][playerId]) {
      state[to][playerId].identity = { ...state[lobby][playerId].identity }; 
    }
    /* ------------------------------ */

    state.editPlayer = {
      playerId: '',
      lobby: 'players'
    };
  },
  [MutationTypes.TOGGLE_ARCHIVE](state) {
    state.isArchive = !state.isArchive;
  },
  [MutationTypes.TOGGLE_BACKUP](state) {
    state.showBackup = !state.showBackup;
  },
  [MutationTypes.TOGGLE_BALANCE](state) {
    state.isBalance = !state.isBalance;
  },
  [MutationTypes.SAVE_TO_ARCHIVE](state) {
    const date = new Date();
    const name = date.toDateString();
    const teams = cloneDeep(state.teams);
    const players = cloneDeep(state.players);
    const reservedPlayers = cloneDeep(state.reservedPlayers);

    state.archive.push({
      date,
      name,
      teams,
      players,
      reservedPlayers,
    });
  },
  [MutationTypes.TOGGLE_SELECTION](state) {
    state.isSelection = !state.isSelection;
  },
  [MutationTypes.TOGGLE_BALANCER_SR](state) {
    state.showBalancerSR = !state.showBalancerSR;
  },
  [MutationTypes.SELECT_PLAYERS](state, { playerIds, lobby }) {
    playerIds.forEach((player) => { state.selectPlayers[lobby][player] = !state.selectPlayers[lobby][player]; });
  },
  [MutationTypes.SELECT_ARCHIVE](state, id) {
    const players = cloneDeep(state.archive[id].players);
    const teams = cloneDeep(state.archive[id].teams);
    const reservedPlayers = cloneDeep(state.archive[id].reservedPlayers);

    state.teams = teams;
    state.players = players;
    state.reservedPlayers = reservedPlayers;
  },
  [MutationTypes.ADD_PLAYER](state, { player, lobby = 'players' }) {
    state[lobby][player.identity.uuid] = player;
  },
  [MutationTypes.ADD_RESERVE](state, uuid) {
    state.reservedPlayers.push(uuid);
  },
  [MutationTypes.ADD_PLAYERS](state, { players, lobby = 'players' }) {
    
    state[lobby] = { ...state[lobby], ...players };
    console.log('asdfg', JSON.stringify(state[lobby]));
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
  [MutationTypes.EMPTY_TEAMS](state) {
    for (let i = 0; i < state.teams.length; i += 1) {
      const players = state.teams[i].members.map(member => member.uuid);
      state.reservedPlayers.push(...players);
      state.teams[i].members = [];
    }
  },
  [MutationTypes.IMPORT_PLAYERS](state, { players, lobby = 'players' }) {
    state[lobby] = { ...players, ...state[lobby] };
  },
  [MutationTypes.SET_BALANCER_OPTIONS](state, options) {
    state.balancerOptions = options;
  },
  [MutationTypes.IMPORT_ARCHIVE](state, data) {
    state.archive.push(data);
  },
  [MutationTypes.UPDATE_STATS](state, { uuid, stats, lobby = 'players' }) {
    state[lobby][uuid].stats = stats;
  },
  [MutationTypes.EDIT_PLAYER](state, { playerId, lobby = 'players' }) {
    state.editPlayer = {
      playerId,
      lobby
    };
  },
  [MutationTypes.DELETE_PLAYER](state, { playerId, lobby = 'players' }) {
    delete state[lobby][playerId];
  },
  [MutationTypes.DELETE_PLAYERS](state, lobby = 'players') {
    state[lobby] = {};
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
  [MutationTypes.ADD_TEAMPLAYER](
    state,
    { playerName, teamUuid, playerId, role, roleName, primary, secondary }
  ) {
    const teamF = state.teams.findIndex(team => team.uuid === teamUuid);

    if (teamF !== -1) {
      state.teams[teamF].members.push({
        uuid: playerId,
        name: playerName,
        rank: role.rank,
        role: roleName,
        primary,
        secondary,
      });
      const { avgSr, totalSr } = TObj.calcStats(state.teams[teamF]);
      state.teams[teamF].avgSr = avgSr;
      state.teams[teamF].totalSr = totalSr;
    }
  },
  [MutationTypes.ASSIGN_CAPTAINS](state, minSR) {
    const players = Object.entries(state.players);
    const captainsCount = Math.floor(players.length / PLAYERS_IN_TEAM);
    const eligible = players
      .filter(([, player]) => PObj.getTopRank(player) >= minSR)
      .sort(([, player], [, player2]) => PObj.getTopRank(player2) - PObj.getTopRank(player));

    let i = 0;
    while (i < captainsCount && i < eligible.length) {
      const [uuid] = eligible[i];
      state.players[uuid].identity.isCaptain = true;
      i += 1;
    }
  },
  [MutationTypes.REMOVE_FROM_ARCHIVE](state, id) {
    state.archive.splice(id, 1);
  },
  [MutationTypes.ASSIGN_SQUIRES](state, maxSR) {
    const players = Object.entries(state.players);
    const squiresCount = Math.floor(players.length / PLAYERS_IN_TEAM);
    const eligible = players
      .filter(([, player]) => PObj.getTopRank(player) <= maxSR)
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
    Object.entries(state.players)
      .filter(([, player]) => player.identity.isCaptain === true)
      .forEach(([uuid]) => {
        state.players[uuid].identity.isCaptain = false;
      });
  },
  [MutationTypes.CLEAR_SQUIRES](state) {
    Object.entries(state.players)
      .filter(([, player]) => player.identity.isSquire === true)
      .forEach(([uuid]) => {
        state.players[uuid].identity.isSquire = false;
      });
  },
  [MutationTypes.CLEAR_ALL_EXTRA](state) {
    Object.entries(state.players)
      .filter(
        ([, player]) => player.identity.isSquire === true || player.identity.isCaptain === true
      )
      .forEach(([uuid]) => {
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
