import './style.css';

import * as sim from 'lib-simulation-wasm';

const viewport = document.getElementById('viewport') as HTMLCanvasElement;
const ctxt = viewport.getContext('2d')!;
function drawTriangle(ctxt, x, y, size, rotation) {
  ctxt.beginPath();
  ctxt.moveTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5
  );
  ctxt.lineTo(
    x - Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size
  );
  ctxt.lineTo(
    x - Math.sin(rotation + (4.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * size
  );
  ctxt.lineTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5
  );

  ctxt.fillStyle = 'rgb(0, 0, 225)';
  ctxt.fill();
  ctxt.strokeStyle = 'rgb(255, 255, 255)';
  ctxt.lineWidth = 1;
  ctxt.stroke();
}

function drawCircle(ctxt, x, y, size) {
  ctxt.beginPath();
  ctxt.arc(x, y, size, 0, 2 * Math.PI);
  ctxt.fillStyle = 'rgb(0, 225, 0)';
  ctxt.fill();
}

const simulation = new sim.Simulation();

const world = simulation.world();
console.log(world);

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

ctxt.scale(viewportScale, viewportScale);

function redraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

  simulation.step();
  for (const food of simulation.world().foods) {
    drawCircle(
      ctxt,
      food.x * viewportWidth,
      food.y * viewportHeight,
      (0.01 / 2) * viewportWidth
    );
  }

  for (const animal of simulation.world().animals) {
    drawTriangle(
      ctxt,
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation
    );
  }

  // requestAnimationFrame() schedules code only for the next frame.
  //
  // Because we want for our simulation to continue forever, we've
  // gotta keep re-scheduling our function:
  requestAnimationFrame(redraw);
}

redraw();
