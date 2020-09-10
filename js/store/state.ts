import { Players } from '../objects/player';

export const STORAGE_KEY = 'owbalancer';

const storage = JSON.parse(window.localStorage.getItem(STORAGE_KEY) || '{}');

const players: Players = storage.players || {};

export const state = {
  editPlayer: '',
  players,
};

export type State = typeof state;
