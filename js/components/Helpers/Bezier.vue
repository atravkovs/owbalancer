<template>
  <canvas
    ref="canva"
    :width="width"
    :height="height"
    @dblclick="dbclick"
    @mousedown="selectActive"
    @mouseup="deselect"
    @mousemove="move"
    >Your browser does not support the HTML5 canvas tag.</canvas
  >
</template>

<script lang="ts">
import { defineComponent, onMounted, PropType, ref, watch } from 'vue';
import { Point, Points, defaultPoints } from '@/objects/bezier';

const offset = 16;
const width = 900;
const height = 150;
const halfHeight = height / 2;

const normalize = (p: Point): Point => {
  return {
    x: (p.x - offset) / (width - offset),
    y: (height - p.y - halfHeight) / halfHeight,
  };
};

const denormalize = (p: Point): Point => {
  return {
    x: p.x * (width - offset) + offset,
    y: height - p.y * halfHeight - halfHeight,
  };
};

const denormalizePoints = (points: Points): Points => {
  return points.map(p => ({ position: denormalize(p.position), control: denormalize(p.control) }));
};

const drawGrid = (c: HTMLCanvasElement) => {
  const ctx = c.getContext('2d');
  if (!ctx) return;

  const percent = 50;
  const ySteps = 10;
  const xSteps = 50;

  ctx.clearRect(0, 0, width, height);
  ctx.strokeStyle = '#000';
  ctx.fillStyle = '#000';

  /* Reload icon */
  ctx.font = '14px Arial';
  ctx.fillText('↻', width - 14, 14);

  ctx.font = '9px Arial';
  /* Axis */
  ctx.lineWidth = 0.75;
  ctx.beginPath();
  ctx.moveTo(0, halfHeight);
  ctx.lineTo(width, halfHeight);
  ctx.stroke();

  ctx.beginPath();
  ctx.moveTo(offset, height);
  ctx.lineTo(offset, 0);
  ctx.stroke();

  /* Value Lines */
  ctx.lineWidth = 0.8;
  ctx.strokeStyle = '#adadad';
  for (let i = 0, j = percent; i <= height; i += height / ySteps, j -= ySteps) {
    let shift = -2;
    let shiftX = 1;

    if (j < 0) shift = 8;
    if (j > 0) shiftX = 2;

    if (i !== halfHeight && Math.abs(j) !== halfHeight) {
      ctx.fillText(`${j}`, shiftX, i + shift);
      ctx.beginPath();
      ctx.moveTo(12, i);
      ctx.lineTo(offset, i);
      ctx.stroke();
    }
  }

  ctx.lineWidth = 2;
  for (let i = offset, j = 0; i <= width; i += (width - offset) / xSteps, j += (5000 / xSteps)) {
    if (j % 500 === 0) {
      ctx.fillText(`${j}`, i - 6, halfHeight + 10);
      
      ctx.beginPath();
      ctx.moveTo(i, halfHeight - 3);
      ctx.lineTo(i, halfHeight + 3);
      ctx.stroke();
    }
  }

  ctx.lineWidth = 0.3;
  for (let i = offset; i <= width; i += (width - offset) / xSteps) {
    ctx.beginPath();
    ctx.moveTo(i, 0);
    ctx.lineTo(i, height);
    ctx.stroke();
  }

  for (let i = 0; i <= height; i += height / ySteps) {
    if (i !== 75) {
      ctx.beginPath();
      ctx.moveTo(0, i);
      ctx.lineTo(width, i);
      ctx.stroke();
    }
  }
};

const drawBezier = (c: HTMLCanvasElement, points: Points) => {
  const ctx = c.getContext('2d');
  if (!ctx) return;

  ctx.lineWidth = 1.2;
  ctx.strokeStyle = '#4334eb';

  for (let i = 1; i < points.length; i += 1) {
    const current = points[i];
    const previous = points[i - 1];

    ctx.beginPath();
    ctx.moveTo(previous.position.x, previous.position.y);
    ctx.bezierCurveTo(
      previous.position.x + previous.control.x,
      previous.position.y + previous.control.y,
      current.position.x - current.control.x,
      current.position.y - current.control.y,
      current.position.x,
      current.position.y
    );
    ctx.stroke();
  }

  ctx.fillStyle = '#990000';
  ctx.strokeStyle = '#000';
  ctx.lineWidth = 0.8;
  for (let i = 0; i < points.length; i += 1) {
    const current = points[i];
    ctx.fillRect(current.position.x - 2, current.position.y - 2, 4, 4);

    ctx.beginPath();
    ctx.arc(
      current.position.x + current.control.x,
      current.position.y + current.control.y,
      3,
      0,
      2 * Math.PI
    );
    ctx.stroke();

    ctx.beginPath();
    ctx.arc(
      current.position.x - current.control.x,
      current.position.y - current.control.y,
      3,
      0,
      2 * Math.PI
    );
    ctx.stroke();
  }

  ctx.lineWidth = 0.6;
  ctx.strokeStyle = '#444';
  for (let i = 0; i < points.length; i += 1) {
    const current = points[i];

    ctx.beginPath();
    ctx.moveTo(current.position.x, current.position.y);
    ctx.lineTo(current.position.x + current.control.x, current.position.y + current.control.y);
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(current.position.x, current.position.y);
    ctx.lineTo(current.position.x - current.control.x, current.position.y - current.control.y);
    ctx.stroke();
  }
};

const inArea = (
  x: number,
  y: number,
  areaX: number,
  areaY: number,
  areaWidth: number,
  areaHeight: number
): boolean => {
  if (x < areaX) return false;
  if (y < areaY) return false;

  if (x > areaX + areaWidth) return false;
  if (y > areaY + areaHeight) return false;

  return true;
};

export default defineComponent({
  name: 'Bezier',
  props: {
    modelValue: Array as PropType<Points>,
  },
  setup(props, { emit }) {
    // eslint-disable-next-line
    const canva: any = ref(null);

    let points: Points = defaultPoints();

    if (props.modelValue && Array.isArray(props.modelValue)) points = [...props.modelValue];

    let activePoint = -1;
    let activeBullet = { point: -1, bullet: 0 };

    const updateCanva = () => {
      const c = canva.value as HTMLCanvasElement;
      drawGrid(c);
      drawBezier(c, denormalizePoints(points));
    };

    const reset = () => {
      points = defaultPoints();

      emit('update:modelValue', points);
    };

    onMounted(() => {
      updateCanva();
    });

    watch(
      () => props.modelValue,
      (p) => {
        if (p && Array.isArray(p)) points = [...p];
        updateCanva();
      } 
    );

    const dbclick = (e: MouseEvent) => {
      const dPoints = denormalizePoints(points);

      const found = dPoints.findIndex(point =>
        inArea(e.offsetX, e.offsetY, point.position.x - 3, point.position.y - 3, 6, 6)
      );

      if (found !== -1) {
        if (found !== 0 && found !== points.length - 1) {
          points.splice(found, 1);
          emit('update:modelValue', points);
        }

        updateCanva();
        return;
      }

      const [first] = dPoints;
      const last = dPoints[points.length - 1];
      const nOffset = normalize({ x: e.offsetX, y: e.offsetY });

      if (e.offsetX > first.position.x && e.offsetX < last.position.x)
        points.push({
          position: nOffset,
          control: normalize({ x: 5, y: 0 }),
        });
      points.sort((a, b) => a.position.x - b.position.x);
      emit('update:modelValue', points);
      updateCanva();
    };

    const selectActive = (e: MouseEvent) => {
      const dPoints = denormalizePoints(points);

      const firstPoint = dPoints.findIndex(point =>
        inArea(
          e.offsetX,
          e.offsetY,
          point.position.x + point.control.x - 3,
          point.position.y + point.control.y - 3,
          6,
          6
        )
      );

      if (firstPoint > -1) activeBullet = { point: firstPoint, bullet: 1 };

      const secondPoint = dPoints.findIndex(point =>
        inArea(
          e.offsetX,
          e.offsetY,
          point.position.x - point.control.x - 3,
          point.position.y - point.control.y - 3,
          6,
          6
        )
      );

      if (secondPoint > -1) activeBullet = { point: secondPoint, bullet: -1 };

      if (activeBullet.point > -1) return;

      activePoint = dPoints.findIndex(point =>
        inArea(e.offsetX, e.offsetY, point.position.x - 3, point.position.y - 3, 6, 6)
      );

      if (activePoint > -1) return;

      if (inArea(e.offsetX, e.offsetY, width - 14, 0, 14, 14)) {
        reset();
        updateCanva();
      }
    };

    const deselect = () => {
      emit('update:modelValue', points);
      activePoint = -1;
      activeBullet = { point: -1, bullet: 0 };
    };

    const move = (e: MouseEvent) => {
      const mOffset = normalize({ x: e.offsetX, y: e.offsetY });
      const dPoints = denormalizePoints(points);

      if (activePoint !== -1) {
        if (activePoint !== 0 && activePoint !== points.length - 1)
          points[activePoint].position.x = mOffset.x;
        points[activePoint].position.y = mOffset.y;
        updateCanva();
      }

      if (activeBullet.point !== -1) {
        points[activeBullet.point].control = normalize({
          x: activeBullet.bullet * (e.offsetX - dPoints[activeBullet.point].position.x),
          y: activeBullet.bullet * (e.offsetY - dPoints[activeBullet.point].position.y),
        });
        updateCanva();
      }
    };

    return { canva, dbclick, selectActive, deselect, move, width, height };
  },
});
</script>

<style scoped>
canvas {
  border: 1px solid #d3d3d3;
}
</style>
