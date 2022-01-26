import { Teams } from '@/objects/team';
import { Archive } from '@/objects/archive';
import { defaultPoints } from '@/objects/bezier';
import { Results, BalancerOptions } from '@/objects/balance';
import { LobbyType, Players, ReservedPlayers } from '@/objects/player';

export const STORAGE_KEY = 'owbalancer';

const storage = JSON.parse(window.localStorage.getItem(STORAGE_KEY) || '{}');

const teams: Teams = storage.teams || [];
const archive: Archive = storage.archive || [];
const players: Players = storage.players || {};
const backup: Players = storage.backup || {};
const reservedPlayers: ReservedPlayers = storage.reservedPlayers || [];
const balancerResults: Results = [];

// Migration
if (storage.balancerOptions && Array.isArray(storage.balancerOptions.adjustSr.tank)) {
  storage.balancerOptions.adjustSr = {
    isEnabled: false,
    tank: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
    support: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
    dps: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
  };
}

const balancerOptions: BalancerOptions = storage.balancerOptions || {
  range: 30,
  triesCount: 25,
  adjustSr: {
    isEnabled: false,
    tank: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
    support: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
    dps: {
      any: defaultPoints(),
      primary: defaultPoints(),
      secondary: defaultPoints(),
    },
  },
  lowRankLimiter: false,
  dispersionMinimizer: false,
  disallowSecondaryRoles: false,
};

const editPlayer: { playerId: string; lobby: LobbyType } = {
  playerId: '',
  lobby: 'players'
};

const selectPlayers: {
  players: {
    [key: string]: boolean;
  };
  backup: {
    [key: string]: boolean;
  };
} = {
  players: {},
  backup: {},
};

export const state = {
  teams,
  backup,
  players,
  archive,
  editPlayer,
  selectPlayers,
  balancerOptions,
  balancerResults,
  reservedPlayers,
  isArchive: false,
  isBalance: false,
  isSelection: false,
  showBalancerSR: false,
  showBackup: storage.showBackup ?? false,
};

export type State = typeof state;
