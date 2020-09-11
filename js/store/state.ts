import { Teams } from '@/objects/team';
import { Players, ReservedPlayers } from '@/objects/player';

export const STORAGE_KEY = 'owbalancer';

const storage = JSON.parse(window.localStorage.getItem(STORAGE_KEY) || '{}');

const players: Players = storage.players || {};
const teams: Teams = storage.teams || [];
const reservedPlayers: ReservedPlayers = storage.reservedPlayers || [];

export const state = {
  teams,
  players,
  editPlayer: '',
  reservedPlayers,
};

export type State = typeof state;
