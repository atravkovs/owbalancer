import { Teams } from '@/objects/team';
import { Players, ReservedPlayers } from '@/objects/player';

export const STORAGE_KEY = 'owbalancer';

const storage = JSON.parse(window.localStorage.getItem(STORAGE_KEY) || '{}');

const teams: Teams = storage.teams || [];
const players: Players = storage.players || {};
const reservedPlayers: ReservedPlayers = storage.reservedPlayers || [];

export const state = {
  teams,
  players,
  editPlayer: '',
  reservedPlayers,
  isBalance: false,
  showBalancerSR: false,
};

export type State = typeof state;
