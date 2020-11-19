import cloneDeep from 'lodash/cloneDeep';

export type Point = {
  x: number;
  y: number;
};

export type BezierPoint = {
  position: Point;
  control: Point;
};

export type Points = BezierPoint[];

export const defaultPoints = (): Points => {
  const points = [
    { position: { x: 0, y: 0 }, control: { x: 0.05982905982905983, y: 1 } },
    { position: { x: 1, y: 0 }, control: { x: 0.05982905982905983, y: 1 } },
  ];

  return cloneDeep(points);
};
